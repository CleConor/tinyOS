#![no_std]
#![no_main]

use core::ptr::write_bytes;

unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
    static __stack_top: u8;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

unsafe fn zero_bss() {
    unsafe {
        let start = &raw mut __bss as *mut u8 as usize;
        let end = &raw mut __bss_end as *mut u8 as usize;
        write_bytes(start as *mut u8, 0, end - start);
    }
}

#[unsafe(no_mangle)] // keep symbol name rust_main
pub unsafe extern "C" fn rust_main() -> ! {
    unsafe { zero_bss() };
    loop {} // the toy kernel does nothing yet
}

core::arch::global_asm!(
    ".section .text.boot",
    ".globl  boot",
    "boot:",
    "  la sp,  __stack_top", // 1) set stack
    "  j  rust_main",        // 2) jump into Rust
);
