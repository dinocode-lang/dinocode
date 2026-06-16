// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/types/token_value.rs
//  Desc:       Token value types.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Integer(i64),
    BigInt(String),
    Float(f64),
    Bool(bool),
    String(String),
    None,
}

impl TokenValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TokenValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            TokenValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_bigint_string(&self) -> Option<&str> {
        match self {
            TokenValue::BigInt(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            TokenValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TokenValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_identifier(&self) -> Option<String> {
        self.as_str().map(|s| s.to_lowercase())
    }
}
