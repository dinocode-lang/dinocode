use crate::{
    memory::MemoryManager,
    types::{
        DinoRef,
        value_type,
    },
};

impl MemoryManager {
    pub fn get_object_len(&self, handle: u32) -> u32 {
        let slot = self.object_pool.get_slot(handle);
        if slot.kind != value_type::OBJECT { return 0; }
        unsafe { slot.data.object.count }
    }

    pub fn get_object_keys(&self, handle: u32) -> Vec<DinoRef> {
        let slot = self.object_pool.get_slot(handle);
        if slot.kind != value_type::OBJECT { return Vec::new(); }
        
        let mut keys = Vec::new();
        unsafe {
            let object = &slot.data.object;
            let cap = object.capacity as usize;
            for i in 0..cap {
                let entry = &*object.entries.add(i);
                if entry.key != 0 {
                    keys.push(DinoRef::from_raw(entry.key));
                }
            }
        }
        keys
    }

    pub fn get_object_values(&self, handle: u32) -> Vec<DinoRef> {
        let slot = self.object_pool.get_slot(handle);
        if slot.kind != value_type::OBJECT { return Vec::new(); }
        
        let mut values = Vec::new();
        unsafe {
            let object = &slot.data.object;
            let cap = object.capacity as usize;
            for i in 0..cap {
                let entry = &*object.entries.add(i);
                if entry.key != 0 {
                    values.push(entry.value);
                }
            }
        }
        values
    }
}
