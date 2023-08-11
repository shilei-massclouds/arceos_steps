//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

/// A byte-granularity memory allocator based on the [simple_system_allocator].
///
/// [simple_system_allocator]: https://docs.rs/simple_system_allocator/latest/simple_system_allocator/
pub struct SimpleByteAllocator;

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {}
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        todo!();
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!();
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        todo!();
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        todo!();
    }

    fn total_bytes(&self) -> usize {
        todo!();
    }

    fn used_bytes(&self) -> usize {
        todo!();
    }

    fn available_bytes(&self) -> usize {
        todo!();
    }
}
