#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(atlas::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

use x86_64::registers::control::Cr3;

// this function is our entrypoint since the C linker
// looks for a '_start' function by default
#[no_mangle] // don't mangle the name of this function in stack traces
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    atlas::init();

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    atlas::hlt_loop()
}

// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    atlas::hlt_loop();
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    atlas::test_panic_handler(info)
}
