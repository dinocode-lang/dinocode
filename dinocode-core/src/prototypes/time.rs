// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/prototypes/time.rs
//  Desc:       Time prototype
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    memory::types::object_types,
    types::{
        DinoRef,
        value_type,
    },
    errors::{
        Result,
        RuntimeError,
    },
    utils::DinoTime
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
};
use dinocode_platform::{
    time::{Duration, UNIX_EPOCH, Instant, local_now},
    thread,
};
use std::sync::LazyLock;

crate::register_module! {
    name: init_time,
    classes: [Time]
}

static BOOT_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

#[dinoclass(static)]
pub struct Time;

#[dinomethods]
impl Time {
    #[key]
    pub const TIMESTAMP: () = ();

    #[key]
    pub const YEAR: () = ();

    #[key]
    pub const MONTH: () = ();

    #[key]
    pub const DAY: () = ();

    #[key]
    pub const HOUR: () = ();

    #[key]
    pub const MINUTE: () = ();

    #[key]
    pub const SECOND: () = ();

    // constructors
    #[raw]
    pub fn now(runtime: &mut Runtime, _args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let duration = local_now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let timestamp_ms = duration.as_millis() as i64;
        Ok(Self::create_instance(runtime, timestamp_ms))
    }

    #[raw]
    pub fn from_timestamp(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument("from_timestamp"));
        }

        let arg = unsafe { *runtime.memory.stack_ptr.add(args_start + 1) };
        let seconds = arg.try_as_int(&mut runtime.memory)?;
        let timestamp_ms = seconds * 1000;
        Ok(Self::create_instance(runtime, timestamp_ms))
    }

    #[raw]
    pub fn from_date(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 4 {
            return Err(RuntimeError::MissingArgument("from_date"));
        }

        let year_arg = unsafe { *runtime.memory.stack_ptr.add(args_start + 1) };
        let month_arg = unsafe { *runtime.memory.stack_ptr.add(args_start + 2) };
        let day_arg = unsafe { *runtime.memory.stack_ptr.add(args_start + 3) };

        let year = year_arg.try_as_int(&mut runtime.memory)?;
        let month = month_arg.try_as_int(&mut runtime.memory)?;
        let day = day_arg.try_as_int(&mut runtime.memory)?;

        let timestamp_ms = DinoTime::to_timestamp_ms(year, month, day);
        Ok(Self::create_instance(runtime, timestamp_ms))
    }

    #[raw]
    pub fn sleep(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument("sleep"));
        }

        let arg = unsafe { *runtime.memory.stack_ptr.add(args_start + 1) };

        match arg.decode_type() {
            value_type::INT => {
                let seconds = arg.as_int() as u64;
                thread::sleep(Duration::from_secs(seconds));
                Ok(DinoRef::NONE)
            }
            value_type::FLOAT => {
                let seconds = arg.as_float();
                let ms = (seconds * 1000.0) as u64;
                thread::sleep(Duration::from_millis(ms));
                Ok(DinoRef::NONE)
            }
            _ => Err(RuntimeError::WrongArgType { 
                func: "sleep", 
                expected: "number (int or float)" 
            })
        }
    }

    // Instance methods

    #[raw]
    pub fn format(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 {
            return Err(RuntimeError::StackUnderflow);
        }

        let (this, pattern) = if args_count > 1 {
            let stack = runtime.memory.stack();
            (stack[args_start], Some(stack[args_start + 1]))
        } else {
            (runtime.memory.stack()[args_start], None)
        };

        if !this.is_object() {
            return Err(RuntimeError::ExpectedInstance("time"));
        }
        let handle = this.get_object_id();

        let year_ref = runtime.get_property_by_id(handle, Self::YEAR())?;
        let month_ref = runtime.get_property_by_id(handle, Self::MONTH())?;
        let day_ref = runtime.get_property_by_id(handle, Self::DAY())?;
        let hour_ref = runtime.get_property_by_id(handle, Self::HOUR())?;
        let minute_ref = runtime.get_property_by_id(handle, Self::MINUTE())?;
        let second_ref = runtime.get_property_by_id(handle, Self::SECOND())?;

        let year = year_ref.try_as_int(&mut runtime.memory)?;
        let month = month_ref.try_as_int(&mut runtime.memory)? as u64;
        let day = day_ref.try_as_int(&mut runtime.memory)? as u64;
        let hour = hour_ref.try_as_int(&mut runtime.memory)? as u64;
        let minute = minute_ref.try_as_int(&mut runtime.memory)? as u64;
        let second = second_ref.try_as_int(&mut runtime.memory)? as u64;

        let pattern_str = if let Some(p) = pattern {
            p.try_as_string(&mut runtime.memory)?
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
        let formatted = dino_time.format(&pattern_str);
        Ok(runtime.memory.alloc_string(&formatted))
    }

    #[raw]
    pub fn add_seconds(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument("add_seconds"));
        }

        let (this, amount_ref) = {
            let stack = runtime.memory.stack();
            (stack[args_start], stack[args_start + 1])
        };

        if !this.is_object() {
            return Err(RuntimeError::ExpectedInstance("time"));
        }
        let handle = this.get_object_id();

        let timestamp_ref = runtime.get_property_by_id(handle, Self::TIMESTAMP())?;
        let timestamp_ms = timestamp_ref.try_as_int(&mut runtime.memory)?;
        let amount = amount_ref.try_as_int(&mut runtime.memory)?;

        let new_timestamp_ms = timestamp_ms + (amount * 1000);
        Ok(Self::create_instance(runtime, new_timestamp_ms))
    }

    #[raw]
    pub fn perf_counter(_runtime: &mut Runtime, _args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let elapsed = BOOT_TIME.elapsed();
        let seconds = elapsed.as_secs_f64();
        Ok(DinoRef::float(seconds))
    }
}

impl Time {
    pub fn create_instance(runtime: &mut Runtime, timestamp_ms: i64) -> DinoRef {
        let handle = runtime.memory.alloc_object_capacity(8);
        let slot = runtime.memory.object_pool.get_slot_mut(handle);
        slot.subkind = object_types::TIME;

        let dino_time = DinoTime::from_timestamp_ms(timestamp_ms);

        let _ = runtime.memory.set_object_property(handle, Self::TIMESTAMP(), DinoRef::int(timestamp_ms), 0);
        let _ = runtime.memory.set_object_property(handle, Self::YEAR(), DinoRef::int(dino_time.year), 0);
        let _ = runtime.memory.set_object_property(handle, Self::MONTH(), DinoRef::int(dino_time.month as i64), 0);
        let _ = runtime.memory.set_object_property(handle, Self::DAY(), DinoRef::int(dino_time.day as i64), 0);
        let _ = runtime.memory.set_object_property(handle, Self::HOUR(), DinoRef::int(dino_time.hour as i64), 0);
        let _ = runtime.memory.set_object_property(handle, Self::MINUTE(), DinoRef::int(dino_time.minute as i64), 0);
        let _ = runtime.memory.set_object_property(handle, Self::SECOND(), DinoRef::int(dino_time.second as i64), 0);

        if let Some(stack_idx) = Self::get_bootstrap_index() {
            let proto_ref = unsafe { runtime.memory.get_global_variable_unchecked(stack_idx) };
            runtime.memory.object_pool.get_slot_mut(handle).proto = proto_ref;
        }

        DinoRef::object(handle)
    }
}
