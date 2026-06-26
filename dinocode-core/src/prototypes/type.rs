// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/prototypes/type.rs
//  Desc:       Type prototype - type checking and comparison
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    types::{
        DinoRef,
        value_type,
    },
    errors::{
        Result,
        RuntimeError,
        RuntimeErrorType,
    },
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    symbol,
};

crate::register_module! {
    name: init_type,
    classes: [Type]
}

#[dinoclass(static)]
pub struct Type;

#[dinomethods]
impl Type {
    #[raw]
    #[symbol(name="call")]
    pub fn call(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count == 0 {
            return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("type".into())));
        }
        let arg = memory.stack().get(args_start + 1).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        
        Ok(DinoRef::int(arg.decode_type() as i64))
    }

    #[prop(key)]
    pub const INT: DinoRef = DinoRef::int(value_type::INT as i64);

    #[prop(key)]
    pub const FLOAT: DinoRef = DinoRef::int(value_type::FLOAT as i64);

    #[prop(key)]
    pub const STR: DinoRef = DinoRef::int(value_type::STRING as i64);

    #[prop(key)]
    pub const BOOL: DinoRef = DinoRef::int(value_type::BOOL as i64);

    #[prop(key)]
    pub const ARRAY: DinoRef = DinoRef::int(value_type::ARRAY as i64);

    #[prop(key)]
    pub const OBJECT: DinoRef = DinoRef::int(value_type::OBJECT as i64);

    #[prop(key)]
    pub const FUNCTION: DinoRef = DinoRef::int(value_type::FUNCTION as i64);

    #[prop(key)]
    pub const NONE: DinoRef = DinoRef::int(value_type::NONE as i64);

    #[prop(key)]
    pub const BIGINT: DinoRef = DinoRef::int(value_type::BIGINT as i64);

    #[prop(key)]
    pub const SYMBOL: DinoRef = DinoRef::int(value_type::SYMBOL as i64);

    #[raw]
    pub fn name(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count == 0 {
            return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("name".into())));
        }
        let arg = memory.stack().get(args_start + 1).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        
        let vtype = arg.decode_type();
        let key_ref = match vtype {
            value_type::INT => Self::INT(),
            value_type::FLOAT => Self::FLOAT(),
            value_type::STRING => Self::STR(),
            value_type::BOOL => Self::BOOL(),
            value_type::ARRAY => Self::ARRAY(),
            value_type::OBJECT => Self::OBJECT(),
            value_type::FUNCTION => Self::FUNCTION(),
            value_type::NONE => Self::NONE(),
            value_type::BIGINT => Self::BIGINT(),
            value_type::SYMBOL => Self::SYMBOL(),
            _ => return Err(RuntimeError::Typed(RuntimeErrorType::InvalidArgumentValue { 
                func: "type.name".to_string(), 
                message: format!("unknown type id: {}", vtype as i64) 
            })),
        };
        Ok(key_ref)
    }

    #[raw]
    pub fn is_primitive(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count == 0 {
            return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("is_primitive".into())));
        }
        let arg = memory.stack().get(args_start + 1).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        
        let vtype = arg.decode_type();
        let is_primitive = matches!(vtype, 
            value_type::INT | 
            value_type::FLOAT | 
            value_type::BOOL | 
            value_type::STRING | 
            value_type::BIGINT | 
            value_type::NONE | 
            value_type::SYMBOL
        );
        
        Ok(DinoRef::bool(is_primitive))
    }

    #[raw]
    pub fn instance_of(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("instance_of".into())));
        }
        
        let stack = memory.stack();
        let instance = stack.get(args_start + 1).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        let class_ref = stack.get(args_start + 2).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        
        if !class_ref.is_class() {
            return Ok(DinoRef::FALSE);
        }
        
        if !instance.is_object() {
            return Ok(DinoRef::FALSE);
        }
        
        let class_id = class_ref.get_object_id();
        let mut current_handle = instance.get_object_id();
        
        loop {
            let slot = memory.object_pool.get_slot(current_handle);
            let proto = slot.proto;
            
            if proto.is_none() {
                return Ok(DinoRef::FALSE);
            }
            
            if proto.is_class() {
                let proto_id = proto.get_object_id();
                if proto_id == class_id {
                    return Ok(DinoRef::TRUE);
                }
            }
            
            if proto.is_object() {
                current_handle = proto.get_object_id();
            } else {
                return Ok(DinoRef::FALSE);
            }
        }
    }

    #[raw]
    pub fn id(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count == 0 {
            return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("id".into())));
        }
        let arg = memory.stack().get(args_start + 1).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        Ok(DinoRef::int(arg.payload() as i64))
    }
}
