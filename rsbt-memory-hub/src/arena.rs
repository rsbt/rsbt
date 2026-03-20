#![cfg_attr(not(feature = "std"), no_std)]

pub struct MemoryArena {
    buffer: &'static mut [u8],
    offset: usize,
}

impl MemoryArena {
    pub fn new(size: usize) -> Self {
        let mut buffer = vec![0u8; size];
        let ptr = buffer.as_mut_ptr();
        let len = buffer.len();
        core::mem::forget(buffer);
        Self {
            buffer: unsafe { core::slice::from_raw_parts_mut(ptr, len) },
            offset: 0,
        }
    }

    pub fn alloc_str(&mut self, s: &str) -> &mut str {
        let bytes = self.alloc_bytes(s.as_bytes());
        core::str::from_utf8_mut(bytes).expect("valid utf-8")
    }

    pub fn alloc_bytes(&mut self, data: &[u8]) -> &mut [u8] {
        let len = data.len();
        self.align_to(1);

        if self.offset + len > self.buffer.len() {
            panic!("arena exhausted");
        }

        let start = self.offset;
        self.offset += len;

        self.buffer[start..start + len].copy_from_slice(data);
        &mut self.buffer[start..start + len]
    }

    pub fn alloc_slice<T: Copy>(&mut self, slice: &[T]) -> &mut [T] {
        let len = slice.len();
        let align = core::mem::align_of::<T>();
        let size = len * core::mem::size_of::<T>();

        self.align_to(align);

        if self.offset + size > self.buffer.len() {
            panic!("arena exhausted");
        }

        let start = self.offset;
        self.offset += size;

        let dst = &mut self.buffer[start..start + size];
        let dst_ptr = dst.as_mut_ptr() as *mut T;
        unsafe {
            dst_ptr.copy_from_nonoverlapping(slice.as_ptr(), len);
            core::slice::from_raw_parts_mut(dst_ptr, len)
        }
    }

    pub fn alloc<T: Copy>(&mut self, value: T) -> T {
        let size = core::mem::size_of::<T>();
        self.align_to(core::mem::align_of::<T>());

        if self.offset + size > self.buffer.len() {
            panic!("arena exhausted");
        }

        let start = self.offset;
        self.offset += size;

        unsafe {
            let ptr = self.buffer.as_mut_ptr().add(start) as *mut T;
            ptr.write(value);
            ptr.read()
        }
    }

    pub fn alloc_vec<T: Copy>(&mut self) -> ArenaVec<T> {
        ArenaVec::new(self)
    }

    fn align_to(&mut self, align: usize) {
        let rem = self.offset % align;
        if rem > 0 {
            self.offset += align - rem;
        }
    }

    pub fn used(&self) -> usize {
        self.offset
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.offset
    }
}

pub struct ArenaVec<T: Copy> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
    _marker: core::marker::PhantomData<T>,
}

impl<T: Copy> ArenaVec<T> {
    fn new(arena: &mut MemoryArena) -> Self {
        let initial_capacity = 4;
        let size = initial_capacity * core::mem::size_of::<T>();

        let align = core::mem::align_of::<T>();
        arena.align_to(align);

        if arena.offset + size > arena.buffer.len() {
            panic!("arena exhausted");
        }

        let start = arena.offset;
        arena.offset += size;

        Self {
            ptr: unsafe { arena.buffer.as_mut_ptr().add(start) as *mut T },
            len: 0,
            capacity: initial_capacity,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }
        unsafe {
            self.ptr.add(self.len).write(value);
            self.len += 1;
        }
    }

    fn grow(&mut self) {
        self.capacity *= 2;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Copy> Drop for ArenaVec<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.ptr);
        }
    }
}
