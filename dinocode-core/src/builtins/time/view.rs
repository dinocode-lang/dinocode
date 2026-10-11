// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/time/view.rs
//  Desc:       Wrapper for time memory operations
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        manager::MemoryManager,
        types::property_flags::PROP_NORMAL,
    },
    types::DinoRef,
    errors::{Result, RuntimeError},
    utils::DinoTime,
    builtins::{
        time::prototype::Time,
        object::view::ObjectView,
    },
};

#[derive(Clone, Copy)]
pub struct TimeView(pub u32);

impl TimeView {
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
            return Err(RuntimeError::ExpectedInstance("time"));
        }
        Ok(Self(dinoref.get_object_id()))
    }

    pub fn new_from_timestamp(timestamp_ms: i64, memory: &mut MemoryManager) -> Result<Self> {
        let dino_time = DinoTime::from_timestamp_ms(timestamp_ms);

        let elements = [
            Time::TIMESTAMP(), DinoRef::int(timestamp_ms),
            Time::YEAR(), DinoRef::int(dino_time.year),
            Time::MONTH(), DinoRef::int(dino_time.month as i64),
            Time::DAY(), DinoRef::int(dino_time.day as i64),
            Time::HOUR(), DinoRef::int(dino_time.hour as i64),
            Time::MINUTE(), DinoRef::int(dino_time.minute as i64),
            Time::SECOND(), DinoRef::int(dino_time.second as i64),
        ];

        let proto = Time::as_ref(memory);
        let object = ObjectView::new_from_slice(proto, &elements, PROP_NORMAL, memory)?;
        Ok(Self(object.as_id()))
    }

    pub fn new_from_date(year: i64, month: i64, day: i64, memory: &mut MemoryManager) -> Result<Self> {
        let timestamp_ms = DinoTime::to_timestamp_ms(year, month, day);
        Self::new_from_timestamp(timestamp_ms, memory)
    }

    pub fn format(&self, template: Option<DinoRef>, memory: &mut MemoryManager) -> Result<String> {
        let object = ObjectView::from_id(self.0);
        let year_ref = object.get(Time::YEAR(), memory)?;
        let month_ref = object.get(Time::MONTH(), memory)?;
        let day_ref = object.get(Time::DAY(), memory)?;
        let hour_ref = object.get(Time::HOUR(), memory)?;
        let minute_ref = object.get(Time::MINUTE(), memory)?;
        let second_ref = object.get(Time::SECOND(), memory)?;

        let year = year_ref.try_as_int(memory)?;
        let month = month_ref.try_as_int(memory)? as u64;
        let day = day_ref.try_as_int(memory)? as u64;
        let hour = hour_ref.try_as_int(memory)? as u64;
        let minute = minute_ref.try_as_int(memory)? as u64;
        let second = second_ref.try_as_int(memory)? as u64;

        let template_str = if let Some(template_ref) = template {
            template_ref.try_as_string(memory)?
        } else {
            "%Y-%m-%d %H:%M:%S".to_string()
        };

        let dino_time = DinoTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        };
        
        let formatted = dino_time.format(&template_str);
        Ok(formatted)
    }
}
