// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/time/prototype.rs
//  Desc:       Time prototype
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════
use crate::{
    runtime::context::Runtime,
    types::{
        DinoRef,
        value_type,
    },
    errors::{
        Result,
        RuntimeError,
    },
    builtins::{
        time::view::TimeView,
        string::view::StringView,
    },
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
        let time = TimeView::new_from_timestamp(timestamp_ms, &mut runtime.memory)?;
        
        Ok(time.as_ref())
    }

    #[raw]
    pub fn from_timestamp(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "from_timestamp", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let seconds = arg.try_as_int(&mut runtime.memory)?;
        let timestamp_ms = seconds * 1000;
        let time = TimeView::new_from_timestamp(timestamp_ms, &mut runtime.memory)?;
        Ok(time.as_ref())
    }

    #[raw]
    pub fn from_date(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 4 {
            return Err(RuntimeError::MissingArgument { func: "from_date", expected: 3, got: args_count - 1 });
        }
        let (year_arg, month_arg, day_arg) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start + 1),
                runtime.memory.stack.get_unchecked(args_start + 2),
                runtime.memory.stack.get_unchecked(args_start + 3)
            )
        };
        let year = year_arg.try_as_int(&mut runtime.memory)?;
        let month = month_arg.try_as_int(&mut runtime.memory)?;
        let day = day_arg.try_as_int(&mut runtime.memory)?;
        let time = TimeView::new_from_date(year, month, day, &mut runtime.memory)?;
        Ok(time.as_ref())
    }

    #[raw]
    pub fn sleep(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "sleep", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
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
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let time = TimeView::from_ref(this)?;
        let pattern = if args_count > 1 {
            Some(unsafe { runtime.memory.stack.get_unchecked(args_start + 1) })
        } else {
            None
        };
        let formatted = time.format(pattern, &mut runtime.memory)?;
        Ok(StringView::new(&formatted, &mut runtime.memory).as_ref())
    }

    #[raw]
    pub fn add_seconds(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "add_seconds", expected: 1, got: args_count - 1 });
        }
        let (this, amount_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };
        let time = this.as_object()?;
        let timestamp_ref = time.get(Self::TIMESTAMP(), &mut runtime.memory)?;
        let timestamp_ms = timestamp_ref.try_as_int(&mut runtime.memory)?;
        let amount = amount_ref.try_as_int(&mut runtime.memory)?;
        let new_timestamp_ms = timestamp_ms + (amount * 1000);
        let new_time = TimeView::new_from_timestamp(new_timestamp_ms, &mut runtime.memory)?;
        Ok(new_time.as_ref())
    }
    
    #[raw]
    pub fn perf_counter(_runtime: &mut Runtime, _args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let elapsed = BOOT_TIME.elapsed();
        let seconds = elapsed.as_secs_f64();
        Ok(DinoRef::float(seconds))
    }
}
