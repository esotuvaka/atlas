#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(atlas::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

use atlas::memory::{self, BootInfoFrameAllocator};
use bootloader::{entry_point, BootInfo};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

entry_point!(kernel_main);

/// This function is our entrypoint via the C linker.
/// Bootloader crate provides a macro to enforce our FFI
/// entrypoint always has the correct function signature.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    atlas::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) }

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
