use std::ptr::NonNull;

pub struct FixedBufferAllocator<'a> {
    buf: &'a mut [u8],
}

impl<'a> FixedBufferAllocator<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf }
    }

    pub fn allocate(&mut self, size: usize) -> NonNull<[u8]> {
        let ptr = NonNull::new(self.buf[0..size].as_mut_ptr()).unwrap();

        NonNull::slice_from_raw_parts(ptr, size)
    }
}
