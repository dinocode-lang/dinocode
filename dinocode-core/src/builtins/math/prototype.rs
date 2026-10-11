// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/math/prototype.rs
//  Desc:       Math prototype
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    types::DinoRef,
    errors::{Result, RuntimeError},
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
};

crate::register_module! {
    name: init_math,
    classes: [Math]
}

#[dinoclass(static)]
pub struct Math;

#[dinomethods]
impl Math {

    #[prop]
    pub const PI: DinoRef = DinoRef::float(std::f64::consts::PI);

    #[prop]
    pub const E: DinoRef = DinoRef::float(std::f64::consts::E);
    
    #[raw]
    pub fn abs(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "abs", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.abs()))
    }

    #[raw]
    pub fn sqrt(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "sqrt", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.sqrt()))
    }

    #[raw]
    pub fn sin(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "sin", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.sin()))
    }

    #[raw]
    pub fn cos(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "cos", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.cos()))
    }

    #[raw]
    pub fn tan(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "tan", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.tan()))
    }

    #[raw]
    pub fn asin(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "asin", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.asin()))
    }

    #[raw]
    pub fn acos(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "acos", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.acos()))
    }

    #[raw]
    pub fn atan(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "atan", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.atan()))
    }

    #[raw]
    pub fn atan2(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 {
            return Err(RuntimeError::MissingArgument { func: "atan2", expected: 2, got: args_count - 1 });
        }
        let arg1 = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let arg2 = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
        let y = arg1.try_as_float(&mut runtime.memory)?;
        let x = arg2.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(y.atan2(x)))
    }

    #[raw]
    pub fn floor(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "floor", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.floor()))
    }

    #[raw]
    pub fn ceil(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "ceil", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.ceil()))
    }

    #[raw]
    pub fn round(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "round", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.round()))
    }

    #[raw]
    pub fn exp(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "exp", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.exp()))
    }

    #[raw]
    pub fn log(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "log", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.ln()))
    }

    #[raw]
    pub fn log10(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "log10", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.log10()))
    }

    #[raw]
    pub fn log2(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "log2", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.log2()))
    }

    #[raw]
    pub fn pow(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 {
            return Err(RuntimeError::MissingArgument { func: "pow", expected: 2, got: args_count - 1 });
        }
        let arg1 = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let arg2 = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
        let base = arg1.try_as_float(&mut runtime.memory)?;
        let exp = arg2.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(base.powf(exp)))
    }

    #[raw]
    pub fn max(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 {
            return Err(RuntimeError::MissingArgument { func: "max", expected: 2, got: args_count - 1 });
        }
        let arg1 = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let arg2 = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
        let x = arg1.try_as_float(&mut runtime.memory)?;
        let y = arg2.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.max(y)))
    }

    #[raw]
    pub fn min(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 {
            return Err(RuntimeError::MissingArgument { func: "min", expected: 2, got: args_count - 1 });
        }
        let arg1 = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let arg2 = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
        let x = arg1.try_as_float(&mut runtime.memory)?;
        let y = arg2.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.min(y)))
    }

    #[raw]
    pub fn cbrt(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "cbrt", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let x = arg.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.cbrt()))
    }

    #[raw]
    pub fn hypot(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 {
            return Err(RuntimeError::MissingArgument { func: "hypot", expected: 2, got: args_count - 1 });
        }
        let arg1 = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let arg2 = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
        let x = arg1.try_as_float(&mut runtime.memory)?;
        let y = arg2.try_as_float(&mut runtime.memory)?;
        Ok(DinoRef::float(x.hypot(y)))
    }

    #[raw]
    pub fn random(_runtime: &mut Runtime, _args_start: usize, _args_count: usize) -> Result<DinoRef> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use dinocode_platform::time::SystemTime;

        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        let seed = hasher.finish();

        let result = (seed as u64).wrapping_mul(1103515245).wrapping_add(12345);
        Ok(DinoRef::float((result & 0x7fffffff) as f64 / 2147483647.0))
    }
}
