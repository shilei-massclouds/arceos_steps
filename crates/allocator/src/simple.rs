//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;
use core::ptr::NonNull;
use rlsf::Tlsf;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};

/// A TLSF (Two-Level Segregated Fit) memory allocator.
///
/// It's just a wrapper structure of [`rlsf::Tlsf`], with `FLLEN` and `SLLEN`
/// fixed to 28 and 32.
pub struct SimpleByteAllocator {
    inner: Tlsf<'static, u32, u32, 28, 32>, // max pool size: 32 * 2^28 = 8G
    total_bytes: usize,
    used_bytes: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            inner: Tlsf::new(),
            total_bytes: 0,
            used_bytes: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        unsafe {
            let pool = core::slice::from_raw_parts_mut(start as *mut u8, size);
            self.inner
                .insert_free_block_ptr(NonNull::new(pool).unwrap())
                .unwrap();
        }
        self.total_bytes = size;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unsafe {
            let pool = core::slice::from_raw_parts_mut(start as *mut u8, size);
            self.inner
                .insert_free_block_ptr(NonNull::new(pool).unwrap())
                .ok_or(AllocError::InvalidParam)?;
        }
        self.total_bytes += size;
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let ptr = self.inner.allocate(layout).ok_or(AllocError::NoMemory)?;
        self.used_bytes += layout.size();
        Ok(ptr.addr())
    }

    fn dealloc(&mut self, pos: NonZeroUsize, layout: Layout) {
        unsafe {
            self.inner
                .deallocate(NonNull::new_unchecked(pos.get() as _), layout.align())
        }
        self.used_bytes -= layout.size();
    }

    fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    fn available_bytes(&self) -> usize {
        self.total_bytes - self.used_bytes
    }
}
