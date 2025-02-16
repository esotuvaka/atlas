#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function in stack traces
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    // this function is our entrypoint since the C linker
    // looks for a '_start' function by default
    loop {}
}

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("running {} tests", tests.len());
    for test in tests {
        test();
    }
}
