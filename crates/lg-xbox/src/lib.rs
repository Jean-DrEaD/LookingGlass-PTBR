#![no_std]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn _aligned_malloc(size: usize, align: usize) -> *mut u8;
    fn _aligned_free(p: *mut u8);
}

struct XboxAlloc;
unsafe impl GlobalAlloc for XboxAlloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 { _aligned_malloc(l.size(), l.align().max(8)) }
    unsafe fn dealloc(&self, p: *mut u8, _: Layout) { _aligned_free(p) }
}

#[global_allocator]
static A: XboxAlloc = XboxAlloc;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn lg_version() -> u32 { 32 }
