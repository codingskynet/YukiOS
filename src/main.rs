#![no_main]
#![no_std]
#![feature(format_args_nl)]

use core::panic::PanicInfo;

mod console;
mod machine;

unsafe fn kernel_init() -> ! {
    println!("hello, world");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
