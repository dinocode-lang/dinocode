use crate::types::DinoRef;
use crate::errors::RuntimeError;
use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
pub struct Stack {
    pub ptr: *mut DinoRef,
    pub sp: *mut DinoRef,
    pub capacity: usize,
    pub start_ptr: *mut DinoRef,
    pub bp: usize,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        let layout = Layout::array::<DinoRef>(size)
            .expect("Stack capacity overflow");
        let ptr = unsafe { alloc(layout) } as *mut DinoRef;
        
        Self {
            ptr,
            sp: ptr,
            capacity: size,
            start_ptr: ptr,
            bp: 0,
        }
    }

    #[inline(always)]
    fn has_space(&self, additional: usize) -> bool {
        let used = unsafe { self.sp.offset_from(self.ptr) } as usize;
        used + additional <= self.capacity
    }

    pub fn grow(&mut self, new_capacity: usize) {
        let new_layout = Layout::array::<DinoRef>(new_capacity)
            .expect("Stack capacity overflow");
        let new_ptr = unsafe { alloc(new_layout) } as *mut DinoRef;
        
        let used = unsafe { self.sp.offset_from(self.ptr) } as usize;
        if used > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(self.ptr, new_ptr, used);
            }
        }
        
        if self.capacity > 0 {
            let old_layout = Layout::array::<DinoRef>(self.capacity)
                .expect("Stack capacity overflow");
            unsafe { dealloc(self.ptr as *mut u8, old_layout); }
        }
        
        self.ptr = new_ptr;
        self.sp = unsafe { new_ptr.add(used) };
        self.start_ptr = new_ptr;
        self.capacity = new_capacity;
    }

    #[inline(always)]
    pub fn push(&mut self, value: DinoRef) {
        if !self.has_space(1) {
            let new_capacity = (self.capacity * 2).max(256);
            self.grow(new_capacity);
        }
        
        unsafe {
            std::ptr::write(self.sp, value);
            self.sp = self.sp.add(1);
        }
    }

    #[inline(always)]
    pub fn push_n(&mut self, count: usize, value: DinoRef) {
        if count == 0 {
            return;
        }
        if !self.has_space(count) {
            let new_depth = self.depth() + count;
            let new_capacity = (self.capacity * 2).max(new_depth).max(256);
            self.grow(new_capacity);
        }
        
        for _ in 0..count {
            unsafe {
                std::ptr::write(self.sp, value);
                self.sp = self.sp.add(1);
            }
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<DinoRef> {
        if self.depth() == 0 {
            return None;
        }
        unsafe {
            self.sp = self.sp.sub(1);
            Some(std::ptr::read(self.sp))
        }
    }

    #[inline(always)]
    pub fn depth(&self) -> usize {
        unsafe { self.sp.offset_from(self.ptr) as usize }
    }
    
    pub fn extend_from_slice(&mut self, values: &[DinoRef]) {
        for &value in values {
            self.push(value);
        }
    }

    pub unsafe fn as_slice_unchecked(&self) -> &[DinoRef] {
        let depth = self.depth();
        unsafe { std::slice::from_raw_parts(self.ptr, depth) }
    }

    #[inline(always)]
    pub unsafe fn get_slice_unchecked(&self, start: usize, count: usize) -> &[DinoRef] {
        let ptr = self.ptr;
        unsafe { std::slice::from_raw_parts(ptr.add(start), count) }
    }

    #[inline(always)]
    pub fn get_slice(&self, start: usize, count: usize) -> Result<&[DinoRef], RuntimeError> {
        if start + count > self.depth() {
            return Err(RuntimeError::StackUnderflow);
        }
        unsafe { Ok(self.get_slice_unchecked(start, count)) }
    }

    #[inline(always)]
    pub fn move_sp(&mut self, new_sp: usize) {
        unsafe { self.sp = self.ptr.add(new_sp); }
    }

    #[inline(always)]
    pub fn pop_n(&mut self, count: usize) {
        let current_depth = self.depth();
        if count <= current_depth {
            unsafe {
                self.sp = self.sp.sub(count);
            }
        }
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, value: DinoRef) {
        unsafe {
            std::ptr::write(self.sp, value);
            self.sp = self.sp.add(1);
        }
    }

    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> DinoRef {
        unsafe {
            self.sp = self.sp.sub(1);
            std::ptr::read(self.sp)
        }
    }

    #[inline(always)]
    pub unsafe fn pop_n_unchecked(&mut self, count: usize) {
        self.sp = unsafe { self.sp.sub(count) };
    }

    #[inline(always)]
    fn check_index(&self, index: usize) -> Result<(), RuntimeError> {
        if index >= self.depth() {
            return Err(RuntimeError::StackUnderflow);
        }
        Ok(())
    }

    #[inline(always)]
    pub fn peek(&self, offset_from_top: usize) -> Result<DinoRef, RuntimeError> {
        self.check_index(offset_from_top)?;
        unsafe { Ok(self.peek_unchecked(offset_from_top)) }
    }

    #[inline(always)]
    pub unsafe fn peek_unchecked(&self, offset_from_top: usize) -> DinoRef {
        let ptr = unsafe { self.sp.sub(offset_from_top + 1) };
        unsafe { std::ptr::read(ptr) }
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Result<DinoRef, RuntimeError> {
        self.check_index(index)?;
        unsafe { Ok(self.get_unchecked(index)) }
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, value: DinoRef) -> Result<(), RuntimeError> {
        self.check_index(index)?;
        unsafe { self.set_unchecked(index, value) };
        Ok(())
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, index: usize) -> DinoRef {
        let ptr = unsafe { self.ptr.add(index) };
        unsafe { std::ptr::read(ptr) }
    }

    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: DinoRef) {
        let ptr = unsafe { self.ptr.add(index) };
        unsafe { std::ptr::write(ptr, value) }
    }

    #[inline(always)]
    pub unsafe fn get_local_unchecked(&self, var_idx: u32) -> DinoRef {
        let index = self.bp + var_idx as usize;
        unsafe { self.get_unchecked(index) }
    }

    #[inline(always)]
    pub unsafe fn set_local_unchecked(&mut self, var_idx: u32, value: DinoRef) {
        let index = self.bp + var_idx as usize;
        unsafe { self.set_unchecked(index, value) }
    }

    pub fn insert(&mut self, index: usize, value: DinoRef) {
        let depth = self.depth();
        if !self.has_space(1) {
            let new_capacity = (self.capacity * 2).max(256);
            self.grow(new_capacity);
        }
        
        unsafe {
            let src = self.ptr.add(index);
            let dst = src.add(1);
            let count = depth - index;
            if count > 0 {
                std::ptr::copy(src, dst, count);
            }
            std::ptr::write(src, value);
            self.sp = self.sp.add(1);
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::array::<DinoRef>(self.capacity)
                .expect("Stack capacity overflow");
            unsafe { dealloc(self.ptr as *mut u8, layout); }
        }
    }
}

