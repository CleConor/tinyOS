#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt;

struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            putchar(c);
        }
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => {{
        let mut writer = Writer;
        let _ = core::fmt::write(&mut writer, format_args!($($arg)*));
        putchar('\n');
    }};
}

macro_rules! println {
    ($($arg:tt)*) => {{
        print!("{}\n", format_args!($($arg)*));
    }};
}

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

struct SbiRet {
    val: i32,
    err: i32,
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {
        unsafe {
            asm! {"wfi"};
        }
    }
}

unsafe fn sbi_call(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    fid: i32,
    eid: i32,
) -> SbiRet {
    let val: i32;
    let err: i32;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => err,
            inlateout("a1") arg1 => val,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack, preserves_flags),
        );
    }

    SbiRet { val, err }
}

fn putchar(ch: char) {
    unsafe {
        sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1);
    }
}

//no_mangle importante, se non viene riconosciuto come simbolo llvm riconosce il pattern
//di zeroing e costruisce una sua memset che manda a puttane il program counter
//in realtà la memset va bene poichè lo stack è ben formato in precedenza, il problema è che
//llv aggiunge in fase di linking una sezione prima di .text chiamata .eh_frame in ox80200000
//che è dove dovrebbe essere .text.boot quindi il programma non parte mai
//non è possibile usare write_bytes perchè llvm crea una chiamata ricorsiva infinita perchè
//internamente tradurrebbe write_bytes come memset
//80200010 <memset>:
//80200010: 00000317      auipc   t1, 0x0
//80200014: 00030067      jr      t1 <memset>
#[unsafe(no_mangle)]
fn memset(dest: *mut u8, val: u8, count: usize) {
    for i in 0..count {
        unsafe {
            *dest.offset(i as isize) = val;
        }
    }
}

#[unsafe(no_mangle)] //mantiene i simboli (nomi funzioni)
fn kernel_main() {
    let bss_size = unsafe { (&__bss_end as *const u8).offset_from(&__bss as *const u8) } as usize;
    unsafe { memset(&__bss as *const u8 as *mut u8, 0, bss_size) };

    //const HELLO: &str = "hello world \r\n";
    //const HELLO_LEN: usize = HELLO.len();

    /*for i in 0..HELLO_LEN {
        let ch = HELLO.as_bytes()[i] as char;
        putchar(ch);
    }*/

    println!("hello wolrd! {:X}", 32);

    loop {
        unsafe { asm!("wfi") }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
fn boot() -> ! {
    unsafe {
        asm!(
            "mv sp, {stack_top}\n
            j {kernel_main}\n",
            stack_top = in(reg) &__stack_top,
            kernel_main = sym kernel_main,
            options(noreturn),
        );
    }
}
