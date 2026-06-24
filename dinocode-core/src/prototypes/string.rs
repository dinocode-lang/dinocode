// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/prototypes/string.rs
//  Desc:       String prototype - methods available on string objects
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    types::DinoRef,
    errors::{
        Result,
        RuntimeError,
        RuntimeErrorType,
    },
    prototypes::array::Array,
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
    pub fn intern(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let handle = this.decode_index() as usize;
        let hash = memory.ensure_const_hash(this);
        
        if let Some(existing) = memory.get_interned_string_by_hash(hash) {
            return Ok(existing);
        }

        memory.set_interned(handle);
        memory.intern_string_by_hash(hash, this);
        
        Ok(this)
    }

    #[raw]
    #[getter]
    pub fn hash(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let hash_val = memory.ensure_const_hash(this);
        Ok(DinoRef::float(hash_val as f64))
    }

    #[raw]
    #[getter]
    pub fn len(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let handle = this.decode_index();
        let len = memory.get_const_len(handle);
        Ok(DinoRef::int(len as i64))
    }
    
    #[raw]
    pub fn is_empty(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let handle = this.decode_index();
        let len = memory.get_const_len(handle);
        Ok(DinoRef::bool(len == 0))
    }
    
    #[raw]
    pub fn char_at(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("char_at".into()))); }
        
        let (this, idx_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let idx = idx_ref.try_as_int(memory)?;
        
        if idx < 0 { return Ok(DinoRef::NONE); }
        
        let handle = this.decode_index();
        let bytes = memory.get_const_bytes(handle);
        
        let mut char_count = 0;
        let mut byte_pos = 0;
        
        while byte_pos < bytes.len() && char_count < idx as usize {
            let s_slice = std::str::from_utf8(&bytes[byte_pos..]).unwrap_or("");
            if let Some(ch) = s_slice.chars().next() {
                char_count += 1;
                byte_pos += ch.len_utf8();
            } else {
                break;
            }
        }
        
        if byte_pos >= bytes.len() || char_count != idx as usize {
            return Ok(DinoRef::NONE);
        }
        
        let s_slice = std::str::from_utf8(&bytes[byte_pos..]).unwrap_or("");
        if let Some(ch) = s_slice.chars().next() {
            let char_str = ch.to_string();
            Ok(memory.alloc_string(&char_str))
        } else {
            Ok(DinoRef::NONE)
        }
    }
    
    #[raw]
    pub fn concat(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("concat".into()))); }
        
        let (this, other) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let other_str = other.try_as_string(memory)?;
        
        let this_handle = this.decode_index();
        let this_str = memory.get_string(this_handle);
        
        let result = format!("{}{}", this_str, other_str);
        Ok(memory.alloc_string(&result))
    }
    
    #[raw]
    #[symbol(name="in", alias)]
    pub fn contains(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("contains".into()))); }
        
        let (this, substr) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let substr_str = substr.try_as_string(memory)?;
        
        let this_handle = this.decode_index();
        let this_str = memory.get_string(this_handle);
        
        Ok(DinoRef::bool(this_str.contains(&substr_str)))
    }
    
    #[raw]
    pub fn starts_with(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("starts_with".into()))); }
        
        let (this, prefix) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err("Stack underflow".into()); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let prefix_str = prefix.try_as_string(memory)?;
        
        let this_handle = this.decode_index();
        let this_str = memory.get_string(this_handle);
        
        Ok(DinoRef::bool(this_str.starts_with(&prefix_str)))
    }
    
    #[raw]
    pub fn ends_with(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("ends_with".into()))); }
        
        let (this, suffix) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err("Stack underflow".into()); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let suffix_str = suffix.try_as_string(memory)?;
        
        let this_handle = this.decode_index();
        let this_str = memory.get_string(this_handle);
        
        Ok(DinoRef::bool(this_str.ends_with(&suffix_str)))
    }
    
    #[raw]
    pub fn to_uppercase(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let this = {
            let stack = memory.stack();
            if args_start >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            stack[args_start]
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let handle = this.decode_index();
        let content = memory.get_string(handle);
        let upper = content.to_uppercase();
        Ok(memory.alloc_string(&upper))
    }
    
    #[raw]
    pub fn to_lowercase(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let this = {
            let stack = memory.stack();
            if args_start >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            stack[args_start]
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let handle = this.decode_index();
        let content = memory.get_string(handle);
        let lower = content.to_lowercase();
        Ok(memory.alloc_string(&lower))
    }
    
    #[raw]
    pub fn trim(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let (this, chars_arg) = {
            let stack = memory.stack();
            if args_start >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            let c_arg = if args_count > 1 { Some(stack[args_start + 1]) } else { None };
            (stack[args_start], c_arg)
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let trimmed = if let Some(arg) = chars_arg {
            let chars_to_trim = arg.try_as_string(memory)?;
            let handle = this.decode_index();
            let content = memory.get_string(handle);
            content.trim_matches(|c: char| chars_to_trim.contains(c)).to_string()
        } else {
            let handle = this.decode_index();
            let content = memory.get_string(handle);
            content.trim().to_string()
        };
        
        Ok(memory.alloc_string(&trimmed))
    }
    
    #[raw]
    pub fn repeat(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("repeat".into()))); }
        
        let (this, count_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let count = count_ref.try_as_int(memory)?;
        
        if count < 0 { 
            return Err(RuntimeError::Typed(RuntimeErrorType::InvalidArgumentValue { 
                func: "repeat".to_string(), 
                message: "repeat count must be non-negative".to_string() 
            })); 
        }
        
        let handle = this.decode_index();
        let content = memory.get_string(handle);
        
        let result = content.repeat(count as usize);
        Ok(memory.alloc_string(&result))
    }
    
    #[raw]
    pub fn split(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let (this, delimiter_ref) = {
            let stack = memory.stack();
            if args_start >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            let del = if args_count > 1 { Some(stack[args_start + 1]) } else { None };
            (stack[args_start], del)
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let parts: Vec<std::string::String> = if let Some(del_ref) = delimiter_ref {
            let delimiter = del_ref.try_as_string(memory)?;
            let handle = this.decode_index();
            let content = memory.get_string(handle);
            if delimiter.is_empty() {
                content.chars().map(|c| c.to_string()).collect()
            } else {
                content.split(&delimiter).map(|s| s.to_string()).collect()
            }
        } else {
            let handle = this.decode_index();
            let content = memory.get_string(handle);
            content.split_whitespace().map(|s| s.to_string()).collect()
        };
        
        let dinoref_parts: Vec<DinoRef> = parts.iter().map(|part| {
            memory.alloc_string(part)
        }).collect();
        
        Ok(Array::create_from_slice(memory, &dinoref_parts))
    }
    
    #[raw]
    pub fn replace(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("replace".into()))); }
        
        let (this, from_ref, to_ref) = {
            let stack = memory.stack();
            if args_start + 2 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1], stack[args_start + 2])
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let from_str = from_ref.try_as_string(memory)?;
        let to_str = to_ref.try_as_string(memory)?;
        
        let handle = this.decode_index();
        let content = memory.get_string(handle);
        
        let result = content.replace(&from_str, &to_str);
        Ok(memory.alloc_string(&result))
    }
    
    #[raw]
    pub fn substr(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("substring".into()))); }
        
        let (this, start_ref, end_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            let end = if args_count > 2 { Some(stack[args_start + 2]) } else { None };
            (stack[args_start], stack[args_start + 1], end)
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let start = start_ref.try_as_int(memory)?;
        if start < 0 { return Ok(DinoRef::NONE); }
        
        let handle = this.decode_index();
        let content = memory.get_string(handle);
        
        let chars: Vec<char> = content.chars().collect();
        let start_usize = start as usize;
        
        if start_usize >= chars.len() { return Ok(DinoRef::NONE); }
        
        let end_usize = if let Some(e_ref) = end_ref {
            let end = e_ref.try_as_int(memory)?;
            if end < 0 { return Ok(DinoRef::NONE); }
            std::cmp::min(end as usize, chars.len())
        } else {
            chars.len()
        };
        
        if start_usize >= end_usize { return Ok(DinoRef::NONE); }
        
        let result: std::string::String = chars[start_usize..end_usize].iter().collect::<std::string::String>();
        Ok(memory.alloc_string(&result))
    }
    
    #[raw]
    pub fn index_of(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("index_of".into()))); }
        
        let (this, search_ref, start_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            let s_ref = if args_count > 2 { Some(stack[args_start + 2]) } else { None };
            (stack[args_start], stack[args_start + 1], s_ref)
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let search_str = search_ref.try_as_string(memory)?;
        
        let start_pos = if let Some(s_ref) = start_ref {
            let start = s_ref.try_as_int(memory)?;
            let handle = this.decode_index();
            let content = memory.get_string(handle);
            if start < 0 { return Ok(DinoRef::int(-1)); }
            std::cmp::min(start as usize, content.len())
        } else {
            0
        };
        
        let handle = this.decode_index();
        let content = memory.get_string(handle);
        
        let substr = &content[start_pos..];
        if let Some(pos) = substr.find(&search_str) {
            Ok(DinoRef::int((start_pos + pos) as i64))
        } else {
            Ok(DinoRef::int(-1))
        }
    }
    
    #[raw]
    pub fn last_index_of(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("last_index_of".into()))); }
        
        let (this, search_ref, end_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            let e_ref = if args_count > 2 { Some(stack[args_start + 2]) } else { None };
            (stack[args_start], stack[args_start + 1], e_ref)
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let search_str = search_ref.try_as_string(memory)?;
        
        let end_pos = if let Some(e_ref) = end_ref {
            let end = e_ref.try_as_int(memory)?;
            let handle = this.decode_index();
            let content = memory.get_string(handle);
            if end < 0 { return Ok(DinoRef::int(-1)); }
            std::cmp::min(end as usize, content.len())
        } else {
            let handle = this.decode_index();
            let content = memory.get_string(handle);
            content.len()
        };
        
        let handle = this.decode_index();
        let content = memory.get_string(handle);
        
        let substr = &content[..end_pos];
        if let Some(pos) = substr.rfind(&search_str) {
            Ok(DinoRef::int(pos as i64))
        } else {
            Ok(DinoRef::int(-1))
        }
    }
    
    #[raw]
    pub fn pad_left(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("pad_left".into()))); }
        
        let (this, width_ref, pad_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            let pad = if args_count > 2 { Some(stack[args_start + 2]) } else { None };
            (stack[args_start], stack[args_start + 1], pad)
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let handle = this.decode_index();
        
        let width = width_ref.try_as_int(memory)?;
        if width < 0 { 
            return Err(RuntimeError::Typed(RuntimeErrorType::InvalidArgumentValue { 
                func: "pad_left".to_string(), 
                message: "width must be non-negative".to_string() 
            })); 
        }
        
        let pad_char = if let Some(p) = pad_ref {
            let pad_str = p.try_as_string(memory)?;
            pad_str.chars().next().unwrap_or(' ')
        } else {
            ' '
        };
        
        let target_len = width as usize;
        let content_len = memory.get_const_len(handle);
        
        if content_len >= target_len {
            return Ok(DinoRef::string(handle));
        }
        
        let padding = target_len - content_len;
        let content = memory.get_string(handle);
        let result = format!("{}{}", pad_char.to_string().repeat(padding), content);
        Ok(memory.alloc_string(&result))
    }
    
    #[raw]
    pub fn pad_right(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("pad_right".into()))); }
        
        let (this, width_ref, pad_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            let pad = if args_count > 2 { Some(stack[args_start + 2]) } else { None };
            (stack[args_start], stack[args_start + 1], pad)
        };
        
        if !this.is_string() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedStringInstance)); }
        
        let handle = this.decode_index();
        
        let width = width_ref.try_as_int(memory)?;
        if width < 0 { 
            return Err(RuntimeError::Typed(RuntimeErrorType::InvalidArgumentValue { 
                func: "pad_right".to_string(), 
                message: "width must be non-negative".to_string() 
            })); 
        }
        
        let pad_char = if let Some(p) = pad_ref {
            let pad_str = p.try_as_string(memory)?;
            pad_str.chars().next().unwrap_or(' ')
        } else {
            ' '
        };
        
        let target_len = width as usize;
        let content_len = memory.get_const_len(handle);
        
        if content_len >= target_len {
            // Return the original string by allocating a new handle for it
            return Ok(DinoRef::string(handle));
        }
        
        let padding = target_len - content_len;
        let content = memory.get_string(handle);
        let result = format!("{}{}", content, pad_char.to_string().repeat(padding));
        Ok(memory.alloc_string(&result))
    }
}
