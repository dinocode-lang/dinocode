// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/interpreter/core/execution/utils/binary_ops.rs
//  Desc:       Binary operations.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{DinoRef, value_type, opcode, Symbol},
    utils::opcode::opcode_symbol,
    errors::{RuntimeError, Result},
};
use super::{
    comparisons,
    arithmetic,
    helpers::{
        try_magic_method, strings_equal, strings_compare
    }
};
use crate::interpreter::core::Runtime;

#[inline]
pub fn execute_binary_operator(
    a: DinoRef,
    b: DinoRef,
    op: u8,
    runtime: &mut Runtime,
) -> Result<DinoRef> {
    
    match (a.decode_type(), b.decode_type(), op) {
        
        (value_type::INT, value_type::INT, opcode::ADD) => {
            Ok(DinoRef::int(a.as_int().wrapping_add(b.as_int())))
        },
        (value_type::INT, value_type::INT, opcode::SUB) => {
            Ok(DinoRef::int(a.as_int().wrapping_sub(b.as_int())))
        },
        (value_type::INT, value_type::INT, opcode::MUL) => {
            Ok(DinoRef::int(a.as_int().wrapping_mul(b.as_int())))
        },
        (value_type::INT, value_type::INT, opcode::DIV) => {
            //let b_val = b.as_int();
            //if b_val == 0 { return Err(RuntimeError::DivisionByZero); }
            Ok(DinoRef::float(a.as_int() as f64 / b.as_int() as f64))
        },
        (value_type::INT, value_type::INT, opcode::MOD) => {
            let b_val = b.as_int();
            if b_val == 0 { return Err(RuntimeError::DivisionByZero); }
            Ok(DinoRef::int(a.as_int() % b_val))
        },
        (value_type::INT, value_type::INT, opcode::FLOOR_DIV) => {
            Ok(DinoRef::float((a.as_int() as f64 / b.as_int() as f64).floor()))
        },
        (value_type::INT, value_type::INT, opcode::POW) => {
            Ok(DinoRef::float((a.as_int() as f64).powf(b.as_int() as f64)))
        },
        
        (value_type::BIGINT, value_type::BIGINT, opcode::ADD) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            let result = &a_bigint + &b_bigint;
            Ok(runtime.memory.alloc_bigint(&result))
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::SUB) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            let result = &a_bigint - &b_bigint;
            Ok(runtime.memory.alloc_bigint(&result))
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::MUL) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            let result = &a_bigint * &b_bigint;
            Ok(runtime.memory.alloc_bigint(&result))
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::DIV) => {
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            if b_bigint.is_zero() { return Err(RuntimeError::DivisionByZero); }
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let result = &a_bigint / &b_bigint;
            Ok(runtime.memory.alloc_bigint(&result))
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::FLOOR_DIV) => {
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            if b_bigint.is_zero() { return Err(RuntimeError::DivisionByZero); }
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let result = &a_bigint / &b_bigint; // In integer division, floor_div is equivalent to normal div
            Ok(runtime.memory.alloc_bigint(&result))
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::MOD) => {
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            if b_bigint.is_zero() { return Err(RuntimeError::DivisionByZero); }
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let result = &a_bigint % &b_bigint;
            Ok(runtime.memory.alloc_bigint(&result))
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::POW) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            let result = a_bigint.pow(&b_bigint);
            Ok(runtime.memory.alloc_bigint(&result))
        },
        
        (value_type::FLOAT, value_type::FLOAT, opcode::ADD) => {
            Ok(DinoRef::float(a.as_float() + b.as_float()))
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::SUB) => {
            Ok(DinoRef::float(a.as_float() - b.as_float()))
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::MUL) => {
            Ok(DinoRef::float(a.as_float() * b.as_float()))
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::DIV) => {
            Ok(DinoRef::float(a.as_float() / b.as_float()))
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::FLOOR_DIV) => {
            Ok(DinoRef::float((a.as_float() / b.as_float()).floor()))
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::MOD) => {
            Ok(DinoRef::float(a.as_float() % b.as_float()))
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::POW) => {
            Ok(DinoRef::float(a.as_float().powf(b.as_float())))
        },
        
        (value_type::STRING, value_type::STRING, opcode::DOT) => {
            let a_str = runtime.memory.get_string(a.decode_index());
            let b_str = runtime.memory.get_string(b.decode_index());
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(a_str);
            result.push_str(b_str);
            Ok(runtime.memory.alloc_string(&result))
        },
        
        (value_type::STRING, _, opcode::DOT) => {
            let b_str = b.try_as_string(runtime.memory)?;
            let a_str = runtime.memory.get_string(a.decode_index());
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(&a_str);
            result.push_str(&b_str);
            Ok(runtime.memory.alloc_string(&result))
        },
        (_, value_type::STRING, opcode::DOT) => {
            let a_str = a.try_as_string(runtime.memory)?;
            let b_str = runtime.memory.get_string(b.decode_index());
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(&a_str);
            result.push_str(b_str);
            Ok(runtime.memory.alloc_string(&result))
        },
        (_, _, opcode::DOT) => {
            let a_str = a.try_as_string(runtime.memory)?;
            let b_str = b.try_as_string(runtime.memory)?;
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(&a_str);
            result.push_str(&b_str);
            Ok(runtime.memory.alloc_string(&result))
        },
        
        (value_type::INT, value_type::INT, opcode::BIT_AND) => {
            Ok(DinoRef::int(a.as_int() & b.as_int()))
        },
        (value_type::INT, value_type::INT, opcode::BIT_OR) => {
            Ok(DinoRef::int(a.as_int() | b.as_int()))
        },
        (value_type::INT, value_type::INT, opcode::BIT_XOR) => {
            Ok(DinoRef::int(a.as_int() ^ b.as_int()))
        },
        
        (value_type::INT, value_type::INT, opcode::EQ) => {
            Ok(if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::INT, value_type::INT, opcode::NE) => {
            Ok(if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::INT, value_type::INT, opcode::GT) => {
            Ok(if a.as_int() > b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::INT, value_type::INT, opcode::LT) => {
            Ok(if a.as_int() < b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::INT, value_type::INT, opcode::GE) => {
            Ok(if a.as_int() >= b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::INT, value_type::INT, opcode::LE) => {
            Ok(if a.as_int() <= b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        (value_type::BIGINT, value_type::BIGINT, opcode::EQ) => {
            if a.raw() == b.raw() { return Ok(DinoRef::TRUE); }
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            Ok(if a_bigint == b_bigint { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::NE) => {
            if a.raw() == b.raw() { return Ok(DinoRef::FALSE); }
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            Ok(if a_bigint != b_bigint { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::GT) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            Ok(if a_bigint > b_bigint { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::LT) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            Ok(if a_bigint < b_bigint { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::GE) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            Ok(if a_bigint >= b_bigint { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::LE) => {
            let a_bigint = runtime.memory.get_bigint(a.as_bigint());
            let b_bigint = runtime.memory.get_bigint(b.as_bigint());
            Ok(if a_bigint <= b_bigint { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        (value_type::FLOAT, value_type::FLOAT, opcode::EQ) => {
            Ok(if a.as_float() == b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::NE) => {
            Ok(if a.as_float() != b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::GT) => {
            Ok(if a.as_float() > b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::LT) => {
            Ok(if a.as_float() < b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::GE) => {
            Ok(if a.as_float() >= b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::LE) => {
            Ok(if a.as_float() <= b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        (value_type::STRING, value_type::STRING, opcode::EQ) => {
            Ok(if strings_equal(a, b, &runtime.memory) { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::STRING, value_type::STRING, opcode::NE) => {
            Ok(if !strings_equal(a, b, &runtime.memory) { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::STRING, value_type::STRING, opcode::GT) => {
            Ok(if strings_compare(a, b, &runtime.memory).is_gt() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::STRING, value_type::STRING, opcode::LT) => {
            Ok(if strings_compare(a, b, &runtime.memory).is_lt() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::STRING, value_type::STRING, opcode::GE) => {
            let cmp = strings_compare(a, b, &runtime.memory);
            Ok(if cmp.is_gt() || cmp.is_eq() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::STRING, value_type::STRING, opcode::LE) => {
            let cmp = strings_compare(a, b, &runtime.memory);
            Ok(if cmp.is_lt() || cmp.is_eq() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        (value_type::BOOL, value_type::BOOL, opcode::EQ) => {
            Ok(if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BOOL, value_type::BOOL, opcode::NE) => {
            Ok(if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BOOL, value_type::BOOL, opcode::GT) => {
            Ok(if a.as_bool() > b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BOOL, value_type::BOOL, opcode::LT) => {
            Ok(if a.as_bool() < b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BOOL, value_type::BOOL, opcode::GE) => {
            Ok(if a.as_bool() >= b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::BOOL, value_type::BOOL, opcode::LE) => {
            Ok(if a.as_bool() <= b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        (value_type::OBJECT, value_type::OBJECT, opcode::EQ) => {
            if let Some(res) = try_magic_method(a, b, Symbol::EQ, runtime)? { return Ok(res); }
            if let Some(res) = try_magic_method(b, a, Symbol::EQ, runtime)? { return Ok(res); }
            Ok(if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::NE) => {
            Ok(if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        (value_type::ARRAY, value_type::ARRAY, opcode::EQ) => {
            Ok(if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::ARRAY, value_type::ARRAY, opcode::NE) => {
            Ok(if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        (value_type::FUNCTION, value_type::FUNCTION, opcode::EQ) => {
            Ok(if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (value_type::FUNCTION, value_type::FUNCTION, opcode::NE) => {
            Ok(if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        // NONE comparisons
        (value_type::NONE, value_type::NONE, opcode::EQ) => Ok(DinoRef::TRUE),
        (value_type::NONE, value_type::NONE, opcode::NE) => Ok(DinoRef::FALSE),
        (value_type::NONE, value_type::NONE, opcode::GE) => Ok(DinoRef::TRUE),
        (value_type::NONE, value_type::NONE, opcode::LE) => Ok(DinoRef::TRUE),
        (value_type::NONE, value_type::NONE, opcode::GT) => Ok(DinoRef::FALSE),
        (value_type::NONE, value_type::NONE, opcode::LT) => Ok(DinoRef::FALSE),
        
        (value_type::NONE, _, opcode::EQ) => Ok(DinoRef::FALSE),
        (_, value_type::NONE, opcode::EQ) => Ok(DinoRef::FALSE),
        (value_type::NONE, _, opcode::NE) => Ok(DinoRef::TRUE),
        (_, value_type::NONE, opcode::NE) => Ok(DinoRef::TRUE),
        (value_type::NONE, _, opcode::GT) => Ok(DinoRef::FALSE),
        (_, value_type::NONE, opcode::GT) => Ok(DinoRef::FALSE),
        (value_type::NONE, _, opcode::LT) => Ok(DinoRef::FALSE),
        (_, value_type::NONE, opcode::LT) => Ok(DinoRef::FALSE),
        (value_type::NONE, _, opcode::GE) => Ok(DinoRef::FALSE),
        (_, value_type::NONE, opcode::GE) => Ok(DinoRef::FALSE),
        (value_type::NONE, _, opcode::LE) => Ok(DinoRef::FALSE),
        (_, value_type::NONE, opcode::LE) => Ok(DinoRef::FALSE),
        
        // Dynamic comparisons
        (value_type::OBJECT | value_type::ARRAY | value_type::STRING, _
            , opcode::IN) => {
            if let Some(res) = try_magic_method(a, b, Symbol::IN, runtime)? { return Ok(res); }
            Ok(DinoRef::FALSE)
        },
        (_, value_type::OBJECT | value_type::ARRAY | value_type::STRING
            , opcode::IN) => {
            if let Some(res) = try_magic_method(b, a, Symbol::IN, runtime)? { return Ok(res); }
            Ok(DinoRef::FALSE)
        },
        (value_type::OBJECT, _, opcode::EQ) => {
            if let Some(res) = try_magic_method(a, b, Symbol::EQ, runtime)? { return Ok(res); }
            Ok(DinoRef::FALSE)
        },
        (_, value_type::OBJECT, opcode::EQ) => {
            if let Some(res) = try_magic_method(b, a, Symbol::EQ, runtime)? { return Ok(res); }
            Ok(DinoRef::FALSE)
        },

        (a_type, b_type, opcode::EQ) => {
            Ok(if comparisons::dyn_equal(a, b, a_type, b_type, runtime.memory)? { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (a_type, b_type, opcode::NE) => {
            Ok(if !comparisons::dyn_equal(a, b, a_type, b_type, runtime.memory)? { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (a_type, b_type, opcode::GT) => {
            Ok(if comparisons::dyn_greater(a, b, a_type, b_type, runtime.memory)? { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (a_type, b_type, opcode::LT) => {
            Ok(if comparisons::dyn_less(a, b, a_type, b_type, runtime.memory)? { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (a_type, b_type, opcode::GE) => {
            Ok(if comparisons::dyn_greater(a, b, a_type, b_type, runtime.memory)? || comparisons::dyn_equal(a, b, a_type, b_type, runtime.memory)? { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        (a_type, b_type, opcode::LE) => {
            Ok(if comparisons::dyn_less(a, b, a_type, b_type, runtime.memory)? || comparisons::dyn_equal(a, b, a_type, b_type, runtime.memory)? { DinoRef::TRUE } else { DinoRef::FALSE })
        },
        
        // Dynamic arithmetic operations
        (a_type, b_type, opcode::ADD) => {
            arithmetic::dyn_add(a, b, a_type, b_type, runtime.memory)
        },
        (a_type, b_type, opcode::SUB) => {
            arithmetic::dyn_sub(a, b, a_type, b_type, runtime.memory)
        },
        (a_type, b_type, opcode::MUL) => {
            arithmetic::dyn_mul(a, b, a_type, b_type, runtime.memory)
        },
        (a_type, b_type, opcode::DIV) => {
            arithmetic::dyn_div(a, b, a_type, b_type, runtime.memory)
        },
        (a_type, b_type, opcode::MOD) => {
            arithmetic::dyn_mod(a, b, a_type, b_type, runtime.memory)
        },
        (a_type, b_type, opcode::FLOOR_DIV) => {
            arithmetic::dyn_floor_div(a, b, a_type, b_type, runtime.memory)
        },
        (a_type, b_type, opcode::POW) => {
            arithmetic::dyn_pow(a, b, a_type, b_type, runtime.memory)
        },
        
        _ => {
            Err(RuntimeError::InvalidBinaryOperation {
                left: a.type_name(),
                op: opcode_symbol(op),
                right: b.type_name(),
            })
        }
    }
}
