// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/type/prototype.rs
//  Desc:       Type prototype
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
    pub fn name(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "name", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        
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
            _ => return Err(RuntimeError::InvalidArgumentValue {
                func: "type.name",
                message: "unrecognized data type"
            }),
        };
        Ok(key_ref)
    }

    #[raw]
    pub fn is_primitive(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "is_primitive", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        
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
    pub fn instance_of(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 {
            return Err(RuntimeError::MissingArgument { func: "instance_of", expected: 2, got: args_count - 1 });
        }
        
        let (instance, class_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start + 1),
                runtime.memory.stack.get_unchecked(args_start + 2)
            )
        };
        
        if !class_ref.is_class() {
            return Ok(DinoRef::FALSE);
        }
        
        if !instance.is_object() {
            return Ok(DinoRef::FALSE);
        }
        
        let class_id = class_ref.get_object_id();
        let mut current_handle = instance.get_object_id();
        
        loop {
            let slot = runtime.memory.object_pool.get_slot(current_handle);
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
    pub fn id(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "id", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        Ok(DinoRef::int(arg.payload() as i64))
    }

    #[raw]
    #[symbol(name="call")]
    pub fn call(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "type", expected: 1, got: args_count - 1 });
        }
        let arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        
        Ok(DinoRef::int(arg.decode_type() as i64))
    }
}
