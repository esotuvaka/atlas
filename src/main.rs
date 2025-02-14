#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function in stack traces
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    // this function is our entrypoint since the C linker
    // looks for a '_start' function by default
    loop {}
}

// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
