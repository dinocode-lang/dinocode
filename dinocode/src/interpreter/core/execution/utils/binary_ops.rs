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
    runtime::context::Runtime,
    types::{DinoRef, value_type, opcode, Symbol, UserFunction},
    utils::opcode::opcode_symbol,
    errors::{RuntimeError, Result},
    builtins::{
        bigint::view::BigIntView,
        string::view::StringView,
    },
    memory::MemoryManager,
};
use super::{
    comparisons,
    arithmetic,
};

macro_rules! try_method {
    ($a:expr, $symbol:expr, $memory:expr, $functions:expr, $ip:expr) => {
        {
            let object = $a.as_object_unchecked();
            if let Ok(magic_method) = object.get($symbol, $memory) {
                let depth = $memory.stack.depth() - 2;
                let mut ip = $ip;
                let mut runtime = Runtime::new($memory, $functions, &mut ip);
                let func = magic_method.as_function_unchecked();
                func.virtual_call(depth, 2, &mut runtime)?;
                return Ok(ip);
            }
        }
    };
}

macro_rules! try_method_swap {
    ($a:expr, $b:expr, $symbol:expr, $memory:expr, $functions:expr, $ip:expr) => {
        {
            let object = $b.as_object_unchecked();
            if let Ok(magic_method) = object.get($symbol, $memory) {
                let depth = $memory.stack.depth() - 2;
                unsafe {
                    $memory.stack.set_unchecked(depth, $b);
                    $memory.stack.set_unchecked(depth + 1, $a);
                }
                let mut ip = $ip;
                let mut runtime = Runtime::new($memory, $functions, &mut ip);
                let func = magic_method.as_function_unchecked();
                func.virtual_call(depth, 2, &mut runtime)?;
                return Ok(ip);
            }
        }
    };
}

#[inline]
pub fn binary_op(
    op: u8,
    memory: &mut MemoryManager,
    functions: &Vec<UserFunction>,
    ip: usize,
) -> Result<usize> {
    
    let b = unsafe { memory.stack.peek_unchecked(0) };
    let a = unsafe { memory.stack.peek_unchecked(1) };

    macro_rules! unsupported {
        () => {
            return Err(RuntimeError::InvalidBinaryOperation {
                left: a.type_name(),
                op: opcode_symbol(op),
                right: b.type_name(),
            })
        };
    }
    
    let result = match (a.decode_type(), b.decode_type(), op) {
        
        // NONE
        (value_type::NONE, value_type::NONE, opcode::EQ) => DinoRef::TRUE,
        (value_type::NONE, value_type::NONE, opcode::NE) => DinoRef::FALSE,
        (value_type::NONE, value_type::NONE, opcode::GE) => DinoRef::TRUE,
        (value_type::NONE, value_type::NONE, opcode::LE) => DinoRef::TRUE,
        (value_type::NONE, value_type::NONE, opcode::GT) => DinoRef::FALSE,
        (value_type::NONE, value_type::NONE, opcode::LT) => DinoRef::FALSE,
        (value_type::NONE, _, opcode::EQ) => DinoRef::FALSE,
        (_, value_type::NONE, opcode::EQ) => DinoRef::FALSE,
        (value_type::NONE, _, opcode::NE) => DinoRef::TRUE,
        (_, value_type::NONE, opcode::NE) => DinoRef::TRUE,
        (value_type::NONE, _, opcode::GT) => DinoRef::FALSE,
        (_, value_type::NONE, opcode::GT) => DinoRef::FALSE,
        (value_type::NONE, _, opcode::LT) => DinoRef::FALSE,
        (_, value_type::NONE, opcode::LT) => DinoRef::FALSE,
        (value_type::NONE, _, opcode::GE) => DinoRef::FALSE,
        (_, value_type::NONE, opcode::GE) => DinoRef::FALSE,
        (value_type::NONE, _, opcode::LE) => DinoRef::FALSE,
        (_, value_type::NONE, opcode::LE) => DinoRef::FALSE,


        // BOOL
        (value_type::BOOL, value_type::BOOL, opcode::EQ) => {
            if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BOOL, value_type::BOOL, opcode::NE) => {
            if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BOOL, value_type::BOOL, opcode::GT) => {
            if a.as_bool() > b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BOOL, value_type::BOOL, opcode::LT) => {
            if a.as_bool() < b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BOOL, value_type::BOOL, opcode::GE) => {
            if a.as_bool() >= b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BOOL, value_type::BOOL, opcode::LE) => {
            if a.as_bool() <= b.as_bool() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        
        // INT
        (value_type::INT, value_type::INT, opcode::ADD) => {
            DinoRef::int(a.as_int().wrapping_add(b.as_int()))
        },
        (value_type::INT, value_type::INT, opcode::SUB) => {
            DinoRef::int(a.as_int().wrapping_sub(b.as_int()))
        },
        (value_type::INT, value_type::INT, opcode::MUL) => {
            DinoRef::int(a.as_int().wrapping_mul(b.as_int()))
        },
        (value_type::INT, value_type::INT, opcode::DIV) => {
            DinoRef::float(a.as_int() as f64 / b.as_int() as f64)
        },
        (value_type::INT, value_type::INT, opcode::MOD) => {
            let b_val = b.as_int();
            if b_val == 0 { return Err(RuntimeError::DivisionByZero); }
            DinoRef::int(a.as_int() % b_val)
        },
        (value_type::INT, value_type::INT, opcode::FLOOR_DIV) => {
            DinoRef::float((a.as_int() as f64 / b.as_int() as f64).floor())
        },
        (value_type::INT, value_type::INT, opcode::POW) => {
            DinoRef::float((a.as_int() as f64).powf(b.as_int() as f64))
        },
        (value_type::INT, value_type::INT, opcode::BIT_AND) => {
            DinoRef::int(a.as_int() & b.as_int())
        },
        (value_type::INT, value_type::INT, opcode::BIT_OR) => {
            DinoRef::int(a.as_int() | b.as_int())
        },
        (value_type::INT, value_type::INT, opcode::BIT_XOR) => {
            DinoRef::int(a.as_int() ^ b.as_int())
        },
        (value_type::INT, value_type::INT, opcode::EQ) => {
            if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::INT, value_type::INT, opcode::NE) => {
            if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::INT, value_type::INT, opcode::GT) => {
            if a.as_int() > b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::INT, value_type::INT, opcode::LT) => {
            if a.as_int() < b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::INT, value_type::INT, opcode::GE) => {
            if a.as_int() >= b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::INT, value_type::INT, opcode::LE) => {
            if a.as_int() <= b.as_int() { DinoRef::TRUE } else { DinoRef::FALSE }
        },


        // FLOAT
        (value_type::FLOAT, value_type::FLOAT, opcode::ADD) => {
            DinoRef::float(a.as_float() + b.as_float())
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::SUB) => {
            DinoRef::float(a.as_float() - b.as_float())
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::MUL) => {
            DinoRef::float(a.as_float() * b.as_float())
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::DIV) => {
            DinoRef::float(a.as_float() / b.as_float())
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::FLOOR_DIV) => {
            DinoRef::float((a.as_float() / b.as_float()).floor())
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::MOD) => {
            DinoRef::float(a.as_float() % b.as_float())
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::POW) => {
            DinoRef::float(a.as_float().powf(b.as_float()))
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::EQ) => {
            if a.as_float() == b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::NE) => {
            if a.as_float() != b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::GT) => {
            if a.as_float() > b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::LT) => {
            if a.as_float() < b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::GE) => {
            if a.as_float() >= b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::FLOAT, value_type::FLOAT, opcode::LE) => {
            if a.as_float() <= b.as_float() { DinoRef::TRUE } else { DinoRef::FALSE }
        },


        // BIGINT
        (value_type::BIGINT, value_type::BIGINT, opcode::ADD) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            let result = &a_bigint + &b_bigint;
            BigIntView::new(&result, memory).as_ref()
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::SUB) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            let result = &a_bigint - &b_bigint;
            BigIntView::new(&result, memory).as_ref()
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::MUL) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            let result = &a_bigint * &b_bigint;
            BigIntView::new(&result, memory).as_ref()
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::DIV) => {
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if b_bigint.is_zero() { return Err(RuntimeError::DivisionByZero); }
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let result = &a_bigint / &b_bigint;
            BigIntView::new(&result, memory).as_ref()
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::FLOOR_DIV) => {
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if b_bigint.is_zero() { return Err(RuntimeError::DivisionByZero); }
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let result = &a_bigint / &b_bigint;
            BigIntView::new(&result, memory).as_ref()
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::MOD) => {
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if b_bigint.is_zero() { return Err(RuntimeError::DivisionByZero); }
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let result = &a_bigint % &b_bigint;
            BigIntView::new(&result, memory).as_ref()
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::POW) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            let result = a_bigint.pow(&b_bigint);
            BigIntView::new(&result, memory).as_ref()
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::EQ) => {
            if a.raw() == b.raw() { return Ok(ip); }
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if a_bigint == b_bigint { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::NE) => {
            if a.raw() == b.raw() { return Ok(ip); }
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if a_bigint != b_bigint { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::GT) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if a_bigint > b_bigint { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::LT) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if a_bigint < b_bigint { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::GE) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if a_bigint >= b_bigint { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::BIGINT, value_type::BIGINT, opcode::LE) => {
            let a_bigint = a.as_bigint_unchecked().get(memory);
            let b_bigint = b.as_bigint_unchecked().get(memory);
            if a_bigint <= b_bigint { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        

        // STRING
        (value_type::STRING, value_type::STRING, opcode::DOT) => {
            let a_str = a.as_string_unchecked().get(memory);
            let b_str = b.as_string_unchecked().get(memory);
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(a_str);
            result.push_str(b_str);
            StringView::new(&result, memory).as_ref()
        },
        (value_type::STRING, _, opcode::DOT) => {
            let b_str = b.try_as_string(memory)?;
            let a_str = a.as_string_unchecked().get(memory);
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(a_str);
            result.push_str(&b_str);
            StringView::new(&result, memory).as_ref()
        },
        (_, value_type::STRING, opcode::DOT) => {
            let a_str = a.try_as_string(memory)?;
            let b_str = b.as_string_unchecked().get(memory);
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(&a_str);
            result.push_str(b_str);
            StringView::new(&result, memory).as_ref()
        },
        (_, _, opcode::DOT) => {
            let a_str = a.try_as_string(memory)?;
            let b_str = b.try_as_string(memory)?;
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(&a_str);
            result.push_str(&b_str);
            StringView::new(&result, memory).as_ref()
        },
        (value_type::STRING, value_type::STRING, opcode::EQ) => {
            let a_str = a.as_string_unchecked();
            let b_str = b.as_string_unchecked();
            if a_str.eq(b_str, memory) { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::STRING, value_type::STRING, opcode::NE) => {
            let a_str = a.as_string_unchecked();
            let b_str = b.as_string_unchecked();
            if !a_str.eq(b_str, memory) { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::STRING, value_type::STRING, opcode::GT) => {
            let a_str = a.as_string_unchecked();
            let b_str = b.as_string_unchecked();
            if a_str.cmp(b_str, memory).is_gt() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::STRING, value_type::STRING, opcode::LT) => {
            let a_str = a.as_string_unchecked();
            let b_str = b.as_string_unchecked();
            if a_str.cmp(b_str, memory).is_lt() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::STRING, value_type::STRING, opcode::GE) => {
            let a_str = a.as_string_unchecked();
            let b_str = b.as_string_unchecked();
            let cmp = a_str.cmp(b_str, memory);
            if cmp.is_gt() || cmp.is_eq() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::STRING, value_type::STRING, opcode::LE) => {
            let a_str = a.as_string_unchecked();
            let b_str = b.as_string_unchecked();
            let cmp = a_str.cmp(b_str, memory);
            if cmp.is_lt() || cmp.is_eq() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::STRING, _, opcode::IN) => {
            let string = a.as_string_unchecked();
            let substr = b.try_as_string(memory)?;
            if string.contains(&substr, memory) { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        

        // OBJECT
        (value_type::OBJECT, value_type::OBJECT, opcode::ADD) => {
            try_method!(a, Symbol::ADD, memory, functions, ip);
            try_method_swap!(a, b, Symbol::RADD, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::SUB) => {
            try_method!(a, Symbol::SUB, memory, functions, ip);
            try_method_swap!(a, b, Symbol::RSUB, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::MUL) => {
            try_method!(a, Symbol::MUL, memory, functions, ip);
            try_method_swap!(a, b, Symbol::RMUL, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::DIV) => {
            try_method!(a, Symbol::DIV, memory, functions, ip);
            try_method_swap!(a, b, Symbol::RDIV, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::MOD) => {
            try_method!(a, Symbol::MOD, memory, functions, ip);
            try_method_swap!(a, b, Symbol::RMOD, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::POW) => {
            try_method!(a, Symbol::POW, memory, functions, ip);
            try_method_swap!(a, b, Symbol::RPOW, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::ADD) => {
            try_method!(a, Symbol::ADD, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::SUB) => {
            try_method!(a, Symbol::SUB, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::MUL) => {
            try_method!(a, Symbol::MUL, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::DIV) => {
            try_method!(a, Symbol::DIV, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::MOD) => {
            try_method!(a, Symbol::MOD, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::POW) => {
            try_method!(a, Symbol::POW, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::ADD) => {
            try_method_swap!(a, b, Symbol::RADD, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::SUB) => {
            try_method_swap!(a, b, Symbol::RSUB, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::MUL) => {
            try_method_swap!(a, b, Symbol::RMUL, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::DIV) => {
            try_method_swap!(a, b, Symbol::RDIV, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::MOD) => {
            try_method_swap!(a, b, Symbol::RMOD, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::POW) => {
            try_method_swap!(a, b, Symbol::RPOW, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::EQ) => {
            try_method!(a, Symbol::EQ, memory, functions, ip);
            try_method_swap!(a, b, Symbol::EQ, memory, functions, ip);
            if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::NE) => {
            try_method!(a, Symbol::NE, memory, functions, ip);
            try_method_swap!(a, b, Symbol::NE, memory, functions, ip);
            if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::LT) => {
            try_method!(a, Symbol::LT, memory, functions, ip);
            try_method_swap!(a, b, Symbol::GT, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::LE) => {
            try_method!(a, Symbol::LE, memory, functions, ip);
            try_method_swap!(a, b, Symbol::GE, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::GT) => {
            try_method!(a, Symbol::GT, memory, functions, ip);
            try_method_swap!(a, b, Symbol::LT, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, value_type::OBJECT, opcode::GE) => {
            try_method!(a, Symbol::GE, memory, functions, ip);
            try_method_swap!(a, b, Symbol::LE, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::EQ) => {
            try_method!(a, Symbol::EQ, memory, functions, ip);
            DinoRef::FALSE
        },
        (value_type::OBJECT, _, opcode::NE) => {
            try_method!(a, Symbol::NE, memory, functions, ip);
            DinoRef::TRUE
        },
        (value_type::OBJECT, _, opcode::LT) => {
            try_method!(a, Symbol::LT, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::LE) => {
            try_method!(a, Symbol::LE, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::GT) => {
            try_method!(a, Symbol::GT, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::GE) => {
            try_method!(a, Symbol::GE, memory, functions, ip);
            unsupported!();
        },
        (value_type::OBJECT, _, opcode::IN) => {
            try_method!(a, Symbol::IN, memory, functions, ip);
            DinoRef::FALSE
        },
        (_, value_type::OBJECT, opcode::EQ) => {
            try_method_swap!(a, b, Symbol::EQ, memory, functions, ip);
            DinoRef::FALSE
        },
        (_, value_type::OBJECT, opcode::NE) => {
            try_method_swap!(a, b, Symbol::NE, memory, functions, ip);
            DinoRef::TRUE
        },
        (_, value_type::OBJECT, opcode::LT) => {
            try_method_swap!(a, b, Symbol::GT, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::LE) => {
            try_method_swap!(a, b, Symbol::GE, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::GT) => {
            try_method_swap!(a, b, Symbol::LT, memory, functions, ip);
            unsupported!();
        },
        (_, value_type::OBJECT, opcode::GE) => {
            try_method_swap!(a, b, Symbol::LE, memory, functions, ip);
            unsupported!();
        },


        // ARRAY
        (value_type::ARRAY, value_type::ARRAY, opcode::EQ) => {
            if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::ARRAY, value_type::ARRAY, opcode::NE) => {
            if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::ARRAY, _, opcode::IN) => {
            let array = a.as_array_unchecked();
            if array.contains(b, memory) { DinoRef::TRUE } else { DinoRef::FALSE }
        },


        // FUNCTION
        (value_type::FUNCTION, value_type::FUNCTION, opcode::EQ) => {
            if a.raw() == b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (value_type::FUNCTION, value_type::FUNCTION, opcode::NE) => {
            if a.raw() != b.raw() { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        
        // Dynamic comparisons
        (a_type, b_type, opcode::EQ) => {
            if comparisons::dyn_equal(a, b, a_type, b_type, memory)? { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (a_type, b_type, opcode::NE) => {
            if !comparisons::dyn_equal(a, b, a_type, b_type, memory)? { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (a_type, b_type, opcode::GT) => {
            if comparisons::dyn_greater(a, b, a_type, b_type, memory)? { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (a_type, b_type, opcode::LT) => {
            if comparisons::dyn_less(a, b, a_type, b_type, memory)? { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (a_type, b_type, opcode::GE) => {
            if comparisons::dyn_greater(a, b, a_type, b_type, memory)? || comparisons::dyn_equal(a, b, a_type, b_type, memory)? { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        (a_type, b_type, opcode::LE) => {
            if comparisons::dyn_less(a, b, a_type, b_type, memory)? || comparisons::dyn_equal(a, b, a_type, b_type, memory)? { DinoRef::TRUE } else { DinoRef::FALSE }
        },
        
        // Dynamic arithmetic operations
        (a_type, b_type, opcode::ADD) => {
            arithmetic::dyn_add(a, b, a_type, b_type, memory)?
        },
        (a_type, b_type, opcode::SUB) => {
            arithmetic::dyn_sub(a, b, a_type, b_type, memory)?
        },
        (a_type, b_type, opcode::MUL) => {
            arithmetic::dyn_mul(a, b, a_type, b_type, memory)?
        },
        (a_type, b_type, opcode::DIV) => {
            arithmetic::dyn_div(a, b, a_type, b_type, memory)?
        },
        (a_type, b_type, opcode::MOD) => {
            arithmetic::dyn_mod(a, b, a_type, b_type, memory)?
        },
        (a_type, b_type, opcode::FLOOR_DIV) => {
            arithmetic::dyn_floor_div(a, b, a_type, b_type, memory)?
        },
        (a_type, b_type, opcode::POW) => {
            arithmetic::dyn_pow(a, b, a_type, b_type, memory)?
        },
        
        _ => unsupported!()
    };
    
    unsafe {
        memory.stack.pop_n_unchecked(2);
        memory.stack.push_unchecked(result);
    }
    
    Ok(ip)
}
