// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/string/prototype.rs
//  Desc:       String prototype
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    types::DinoRef,
    errors::{
        Result,
        RuntimeError,
    },
    builtins::{
        array::view::ArrayView,
        string::view::StringView,
    },
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    getter,
    symbol,
};

crate::register_module! {
    name: init_string,
    classes: [String]
}

#[dinoclass]
pub struct String;

#[dinomethods]
impl String {
    #[raw]
    pub fn intern(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };        
        let string = this.as_string()?;
        string.intern(&mut runtime.memory)
    }

    #[raw]
    #[getter]
    pub fn hash(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let string = this.as_string()?;
        let hash_val = string.hash(&mut runtime.memory);
        Ok(DinoRef::float(hash_val as f64))
    }

    #[raw]
    #[getter]
    pub fn len(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let string = this.as_string()?;
        let len = string.len(&runtime.memory);
        Ok(DinoRef::int(len as i64))
    }
    
    #[raw]
    pub fn is_empty(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let string = this.as_string()?;
        Ok(DinoRef::bool(string.is_empty(&runtime.memory)))
    }
    
    #[raw]
    pub fn char_at(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "char_at", expected: 1, got: args_count - 1 }); }
        
        let (this, idx_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };
        
        let string = this.as_string()?;
        let idx = idx_ref.try_as_int(&mut runtime.memory)?;
        if let Some(char) = string.char_at(idx, &runtime.memory) {
            Ok(StringView::new(&char.to_string(), &mut runtime.memory).as_ref())
        } else {
            Ok(DinoRef::NONE)
        }
    }
    
    #[raw]
    pub fn concat(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "concat", expected: 1, got: args_count - 1 }); }

        let (this, other) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let other_str = other.try_as_string(runtime.memory)?;
        let string = this.as_string()?;
        let result = string.concat(&other_str, &mut runtime.memory);
        Ok(StringView::new(&result, &mut runtime.memory).as_ref())
    }
    
    #[raw]
    #[symbol(name="in", alias)]
    pub fn contains(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "contains", expected: 1, got: args_count - 1 }); }

        let (this, substr) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let substr_str = substr.try_as_string(runtime.memory)?;
        let string = this.as_string()?;
        Ok(DinoRef::bool(string.contains(&substr_str, &runtime.memory)))
    }
    
    #[raw]
    pub fn starts_with(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "starts_with", expected: 1, got: args_count - 1 }); }

        let (this, prefix) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let prefix_str = prefix.try_as_string(runtime.memory)?;
        let string = this.as_string()?;
        Ok(DinoRef::bool(string.starts_with(&prefix_str, &runtime.memory)))
    }
    
    #[raw]
    pub fn ends_with(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "ends_with", expected: 1, got: args_count - 1 }); }

        let (this, suffix) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let suffix_str = suffix.try_as_string(runtime.memory)?;
        let string = this.as_string()?;
        Ok(DinoRef::bool(string.ends_with(&suffix_str, &runtime.memory)))
    }
    
    #[raw]
    pub fn to_uppercase(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let string = this.as_string()?;
        let upper = string.to_uppercase(&runtime.memory);
        Ok(StringView::new(&upper, &mut runtime.memory).as_ref())
    }
    
    #[raw]
    pub fn to_lowercase(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let string = this.as_string()?;
        let lower = string.to_lowercase(&runtime.memory);
        Ok(StringView::new(&lower, &mut runtime.memory).as_ref())
    }
    
    #[raw]
    pub fn trim(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let string = this.as_string()?;

        let trimmed = if args_count > 1 {
            let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
            let chars_to_trim = arg.try_as_string(runtime.memory)?;
            string.trim(Some(&chars_to_trim), &runtime.memory)
        } else {
            string.trim(None, &runtime.memory)
        };

        Ok(StringView::new(&trimmed, &mut runtime.memory).as_ref())
    }
    
    #[raw]
    pub fn repeat(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "repeat", expected: 1, got: args_count - 1 }); }

        let (this, count_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let count = count_ref.try_as_int(runtime.memory)?;

        if count < 0 {
            return Err(RuntimeError::InvalidArgumentValue {
                func: "repeat",
                message: "repeat count must be non-negative"
            });
        }

        let string = this.as_string()?;
        let result = string.repeat(count as usize, &runtime.memory);
        Ok(StringView::new(&result, &mut runtime.memory).as_ref())
    }
    
    #[raw]
    pub fn split(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        use std::string::String;
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };

        let string = this.as_string()?;

        let parts: Vec<String> = if args_count > 1 {
            let delimiter = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) }.try_as_string(runtime.memory)?;
            string.split(Some(&delimiter), &runtime.memory)
        } else {
            string.split(None, &runtime.memory)
        };

        let dinoref_parts: Vec<DinoRef> = parts.iter().map(|part| {
            StringView::new(part, &mut runtime.memory).as_ref()
        }).collect();

        let array = ArrayView::new_from_slice(&dinoref_parts, &mut runtime.memory)?;
        Ok(array.as_ref())
    }
    
    #[raw]
    pub fn replace(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 { return Err(RuntimeError::MissingArgument { func: "replace", expected: 2, got: args_count - 1 }); }

        let (this, from_ref, to_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1),
                runtime.memory.stack.get_unchecked(args_start + 2)
            )
        };

        let from_str = from_ref.try_as_string(runtime.memory)?;
        let to_str = to_ref.try_as_string(runtime.memory)?;

        let string = this.as_string()?;
        let result = string.replace(&from_str, &to_str, &runtime.memory);
        Ok(StringView::new(&result, &mut runtime.memory).as_ref())
    }
    
    #[raw]
    pub fn substr(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "substring", expected: 1, got: args_count - 1 }); }

        let (this, start_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let start = start_ref.try_as_int(runtime.memory)?;
        if start < 0 { return Ok(DinoRef::NONE); }

        let string = this.as_string()?;

        let end_usize = if args_count > 2 {
            let end_ref = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
            let end = end_ref.try_as_int(runtime.memory)?;
            if end < 0 { return Ok(DinoRef::NONE); }
            Some(end as usize)
        } else {
            None
        };

        if let Some(result) = string.substr(start as usize, end_usize, &runtime.memory) {
            Ok(StringView::new(&result, &mut runtime.memory).as_ref())
        } else {
            Ok(DinoRef::NONE)
        }
    }
    
    #[raw]
    pub fn index_of(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "index_of", expected: 1, got: args_count - 1 }); }

        let (this, search_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let search_str = search_ref.try_as_string(runtime.memory)?;

        let start_pos = if args_count > 2 {
            let start_ref = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
            let start = start_ref.try_as_int(runtime.memory)?;
            if start < 0 { return Ok(DinoRef::int(-1)); }
            let string = this.as_string()?;
            std::cmp::min(start as usize, string.len(&runtime.memory))
        } else {
            0
        };

        let string = this.as_string()?;

        if let Some(pos) = string.index_of(&search_str, start_pos, &runtime.memory) {
            Ok(DinoRef::int(pos as i64))
        } else {
            Ok(DinoRef::int(-1))
        }
    }
    
    #[raw]
    pub fn last_index_of(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "last_index_of", expected: 1, got: args_count - 1 }); }

        let (this, search_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let search_str = search_ref.try_as_string(runtime.memory)?;

        let end_pos = if args_count > 2 {
            let end_ref = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
            let end = end_ref.try_as_int(runtime.memory)?;
            if end < 0 { return Ok(DinoRef::int(-1)); }
            let string = this.as_string()?;
            std::cmp::min(end as usize, string.len(&runtime.memory))
        } else {
            let string = this.as_string()?;
            string.len(&runtime.memory)
        };

        let string = this.as_string()?;

        if let Some(pos) = string.last_index_of(&search_str, end_pos, &runtime.memory) {
            Ok(DinoRef::int(pos as i64))
        } else {
            Ok(DinoRef::int(-1))
        }
    }
    
    #[raw]
    pub fn pad_left(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "pad_left", expected: 1, got: args_count - 1 }); }

        let (this, width_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let width = width_ref.try_as_int(runtime.memory)?;
        if width < 0 {
            return Err(RuntimeError::InvalidArgumentValue {
                func: "pad_left",
                message: "width must be non-negative"
            });
        }

        let pad_char = if args_count > 2 {
            let pad_ref = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
            let pad_str = pad_ref.try_as_string(runtime.memory)?;
            pad_str.chars().next().unwrap_or(' ')
        } else {
            ' '
        };

        let string = this.as_string()?;
        let result = string.pad_left(width as usize, pad_char, &runtime.memory);
        Ok(StringView::new(&result, &mut runtime.memory).as_ref())
    }
    
    #[raw]
    pub fn pad_right(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "pad_right", expected: 1, got: args_count - 1 }); }

        let (this, width_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let width = width_ref.try_as_int(runtime.memory)?;
        if width < 0 {
            return Err(RuntimeError::InvalidArgumentValue {
                func: "pad_right",
                message: "width must be non-negative"
            });
        }

        let pad_char = if args_count > 2 {
            let pad_ref = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
            let pad_str = pad_ref.try_as_string(runtime.memory)?;
            pad_str.chars().next().unwrap_or(' ')
        } else {
            ' '
        };

        let string = this.as_string()?;
        let result = string.pad_right(width as usize, pad_char, &runtime.memory);
        Ok(StringView::new(&result, &mut runtime.memory).as_ref())

    }
}

