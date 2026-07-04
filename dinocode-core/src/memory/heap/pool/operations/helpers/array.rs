use crate::{
    memory::MemoryManager,
    types::{
        DinoRef,
        value_type,
    },
    errors::{Result, RuntimeError},
};
use std::alloc::{Layout, realloc};

impl MemoryManager {
    pub fn get_array_len(&self, handle: u32) -> u32 {
        let slot = self.object_pool.get_slot(handle);
        if slot.kind != value_type::ARRAY { return 0; }
        unsafe { slot.data.array.count }
    }

    pub fn get_array_element(&self, handle: u32, index: u32) -> DinoRef {
        let slot = self.object_pool.get_slot(handle);
        if slot.kind != value_type::ARRAY { return DinoRef::NONE; }
        
        unsafe {
            let array = &slot.data.array;
            if index < array.count {
                std::ptr::read(array.elements.add(index as usize))
            } else {
                DinoRef::NONE
            }
        }
    }
    
    pub fn set_array_element(&mut self, handle: u32, index: u32, value: DinoRef) -> Result<()> {
        let slot = self.object_pool.get_slot_mut(handle);
        if slot.kind != value_type::ARRAY { 
            return Err(RuntimeError::ExpectedInstance("array"));
        }
        
        let cap_needed = index + 1;
        
        let (current_cap, current_ptr) = unsafe {
             let array = &slot.data.array;
             (array.capacity, array.elements)
        };
        
        if cap_needed > current_cap {
            let new_cap = cap_needed.next_power_of_two().max(current_cap * 2);
            let old_layout = Layout::array::<DinoRef>(current_cap as usize).unwrap();
            let new_layout = Layout::array::<DinoRef>(new_cap as usize).unwrap();
            
            unsafe {
                let new_ptr = realloc(current_ptr as *mut u8, old_layout, new_layout.size()) as *mut DinoRef;
                if new_ptr.is_null() { 
                    return Err(RuntimeError::InternalError("Out of memory"));
                }
                
                let slot = self.object_pool.get_slot_mut(handle);
                slot.data.array.elements = new_ptr;
                slot.data.array.capacity = new_cap;
            }
        }
        
        let slot = self.object_pool.get_slot_mut(handle);
        let array = unsafe { &mut slot.data.array };
        
         unsafe {
            std::ptr::write(array.elements.add(index as usize), value);
            if index >= array.count {
                array.count = index + 1;
            }
        }
        
        Ok(())
    }

    pub fn array_pop(&mut self, handle: u32) -> DinoRef {
        let slot = self.object_pool.get_slot_mut(handle);
        if slot.kind != value_type::ARRAY { return DinoRef::NONE; }
        
        unsafe {
            let array = &mut slot.data.array;
            if array.count == 0 { return DinoRef::NONE; }
            
            array.count -= 1;
            let val = std::ptr::read(array.elements.add(array.count as usize));
            val
        }
    }
}
