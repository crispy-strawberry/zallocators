use core::{
    alloc::{Layout, LayoutError},
    ptr::NonNull,
};
use std::vec::Vec as OgVec;

use crate::{AllocError, Allocator};

pub struct Vec<A: Allocator, T> {
    alloc: A,
    len: usize,
    cap: usize,
    ptr: NonNull<T>,
}

impl<A: Allocator, T> Vec<A, T> {
    pub const fn new(alloc: A) -> Self {
        Self {
            alloc,
            len: 0,
            cap: 0,
            ptr: NonNull::dangling(),
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn cap(&self) -> usize {
        self.cap
    }

    pub fn with_capacity(alloc: A, cap: usize) -> Result<Self, AllocError> {
        let layout = Layout::array::<T>(cap).unwrap();
        // std::vec::Vec::try_reserve(, )
        // OgVec::try_reserve(, )
        let ptr = alloc.allocate(layout)?.cast();
        Ok(Self {
            alloc,
            cap,
            ptr,
            len: 0,
        })
    }

    pub const fn allocator(&self) -> &A {
        &self.alloc
    }

    pub const fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub const fn as_mut_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}
