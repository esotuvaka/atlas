#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(atlas::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use alloc::{boxed::Box, vec::Vec};
use atlas::{
    allocator::{self, HEAP_SIZE},
    memory::{self, BootInfoFrameAllocator},
};
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

extern crate alloc;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    atlas::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    atlas::test_panic_handler(info)
}

#[test_case]
/// validate basic allocs and reads
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
/// validate that allocated values read the same value after being written
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i)
    }
    // nth partial sum
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2)
}

#[test_case]
/// validate that allocator reuses freed memory on subsequent allocations
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
