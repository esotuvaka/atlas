pub mod bump;
pub mod linked_list;

use bump::BumpAllocator;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};
use linked_list::LinkedListAllocator;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

/// Wrapper around spin::Mutex to permit trait implementations.
///
/// Enables synchronized interior mutability over `A`.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two (guaranteed via the `GlobalAlloc`
/// trait and its `Layout` parameter). Creates a bitmask to align addresses
/// efficiently.
fn align_up(addr: usize, align: usize) -> usize {
    // SLOW
    // let remainder = addr % align;
    // if remainder == 0 {
    //     addr // addr already aligned
    // } else {
    //     addr - remainder + align
    // }

    // FASTER
    // 1. As a power of two, `align` has a single bit set;                              e.g: 0b000100000
    // 2. This means that `align - 1` has all lower bits set;                           e.g: 0b000011111
    // 3. ! (bitwise NOT) gives a number with all bits set except lower than align; e.g: 0b..11111000000
    // 4. & (bitwise AND) with `!(align - 1)` aligns the address downwards (clears bits lower than align)
    // 5. To align up instead of down, increase `addr` by `align - 1` before performing &
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

// FIXME: requires `heap_start` be `*mut u8` for polymorph with `linked_list_allocator`
// #[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

// #[global_allocator]
// static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        // let heap_start = HEAP_START as *mut u8;
        // ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
        let heap_start = HEAP_START;
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }

    Ok(())
}

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}
