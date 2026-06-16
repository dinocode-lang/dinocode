// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/gc/copying.rs
//  Desc:       Copying GC (Arena only)
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        MemoryManager,
        types::heap_header::IS_INTERNED,
    },
    utils::StringInterner,
    types::{
        DinoRef,
        dinoref::value_type,
    },
};

impl MemoryManager {

    pub fn compact_and_evacuate_heap(&mut self) {
        let mut new_arena = Vec::with_capacity(self.arena.len());

        if self.globals_start > 0 {
            new_arena.extend_from_slice(&self.arena[0..self.globals_start]);
            self.string_interner.truncate_to_globals();
        }

        let globals_start = self.globals_start;
        let depth = self.stack_depth();
        let old_arena = &mut self.arena;
        let interner = &mut self.string_interner;

        for i in 0..depth {
            unsafe {
                let ptr = self.stack_ptr.add(i);
                let val = *ptr;
                let new_val = Self::evacuate(val, old_arena, &mut new_arena, interner, globals_start);
                if val.raw() != new_val.raw() {
                    *ptr = new_val;
                }
            }
        }

        for i in 0..self.object_pool.capacity {
             if self.object_pool.bitmap.get(i) {
                 let slot = self.object_pool.get_slot_mut(i as u32);
                 match slot.kind {
                     value_type::ARRAY => {
                          let array = unsafe { &mut slot.data.array };
                          unsafe {
                              for k in 0..array.count {
                                  let p = array.elements.add(k as usize);
                                  let val = *p;
                                  let new_val = Self::evacuate(val, &mut self.arena, &mut new_arena, &mut self.string_interner, globals_start);
                                  *p = new_val;
                              }
                          }
                     },
                     value_type::OBJECT => {
                          let object = unsafe { &mut slot.data.object };
                          unsafe {
                              for k in 0..object.capacity {
                                  let entry = &mut *object.entries.add(k as usize);
                                  if entry.key != 0 {
                                      let key_ref = DinoRef::from_raw(entry.key);
                                      if key_ref.is_string() { 
                                          let new_key = Self::evacuate(key_ref, &mut self.arena, &mut new_arena, &mut self.string_interner, globals_start);
                                          entry.key = new_key.raw();
                                      }
                                      let new_val = Self::evacuate(entry.value, &mut self.arena, &mut new_arena, &mut self.string_interner, globals_start);
                                      entry.value = new_val;
                                  }
                              }
                          }
                     },
                     _ => {}
                 }
             }
        }

        self.arena = new_arena;
    }

    fn evacuate(value: DinoRef, old_arena: &mut [u8], new_arena: &mut Vec<u8>, new_interner: &mut StringInterner, globals_start: usize) -> DinoRef {
        let vtype = value.decode_type();
        match vtype {

            value_type::STRING | value_type::BIGINT => {
                let offset = value.payload() as usize;

                if offset < globals_start || offset >= old_arena.len() {
                    return value;
                }

                let flags = old_arena[offset + 2];
                if (flags & 0x80) != 0 {
                    let data_start = offset + 8;
                    if data_start + 4 <= old_arena.len() {
                        let new_off_bytes = &old_arena[data_start..data_start + 4];
                        let new_offset = u32::from_le_bytes(new_off_bytes.try_into().unwrap());
                        return value.with_payload(new_offset as u64);
                    }
                }

                let size_bytes = &old_arena[offset + 4..offset + 8];
                let size = u32::from_le_bytes(size_bytes.try_into().unwrap());
                let total_len = 8 + size as usize;

                let new_offset = new_arena.len() as u32;

                if offset + total_len > old_arena.len() {
                    return value;
                }

                new_arena.extend_from_slice(&old_arena[offset..offset + total_len]);

                old_arena[offset + 2] |= 0x80;
                let data_start = offset + 8;
                let new_loc_bytes = new_offset.to_le_bytes();
                old_arena[data_start..data_start + 4].copy_from_slice(&new_loc_bytes);

                let new_dinoref = value.with_payload(new_offset as u64);

                if vtype == value_type::STRING {
                    if (flags & IS_INTERNED) != 0 {
                        let no = new_offset as usize;
                        let hash = u64::from_le_bytes(new_arena[no + 16..no + 24].try_into().unwrap());
                        let string_len = u32::from_le_bytes(new_arena[no + 8..no + 12].try_into().unwrap()) as usize;
                        new_interner.insert(hash, new_dinoref, string_len, new_arena);
                    }
                }

                new_dinoref
            },

            _ => value,

        }
    }
}