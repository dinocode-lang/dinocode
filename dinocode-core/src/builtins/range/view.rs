// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/range/view.rs
//  Desc:       Wrapper for range memory operations
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        manager::MemoryManager,
        types::{
            object_types,
            property_flags::PROP_NORMAL,
        },
    },
    types::DinoRef,
    errors::{Result, RuntimeError},
    builtins::{
        range::prototype::Range,
        object::view::ObjectView,
    },
};

#[derive(Clone, Copy)]
pub struct RangeView(pub u32);

impl RangeView {
    #[inline(always)]
    pub fn as_id(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn as_ref(&self) -> DinoRef {
        DinoRef::object(self.0)
    }

    #[inline(always)]
    pub fn from_ref(dinoref: DinoRef) -> Result<Self> {
        if !dinoref.is_object() {
            return Err(RuntimeError::ExpectedInstance("range"));
        }
        Ok(Self(dinoref.get_object_id()))
    }

    pub fn new(start: i64, stop: i64, step: i64, inclusive: bool, memory: &mut MemoryManager) -> Result<Self> {
        let stop = if inclusive {
            stop.saturating_add(1)
        } else {
            stop
        };

        let proto_ref = Range::as_ref(memory);
        let pairs = [
            Range::START(), DinoRef::int(start),
            Range::STOP(), DinoRef::int(stop),
            Range::STEP_VAL(), DinoRef::int(step),
        ];

        let object = ObjectView::new_from_slice(proto_ref, &pairs, PROP_NORMAL, memory)?;
        
        let slot = memory.object_pool.get_slot_mut(object.as_id());
        slot.subkind = object_types::RANGE;

        Ok(Self(object.as_id()))
    }
}
