// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/types/typeid.rs
//  Desc:       Type identifiers for the DinoCode type system
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeId {
    Number = 0,
    Int = 1,
    Float = 2,
    String = 3,
    Bool = 4,
    BigInt = 5,
}

impl TypeId {
    pub fn from_str(type_str: &str) -> Option<Self> {
        match type_str {
            "number" => Some(TypeId::Number),
            "int" => Some(TypeId::Int),
            "float" => Some(TypeId::Float),
            "str" => Some(TypeId::String),
            "bool" => Some(TypeId::Bool),
            "bigint" => Some(TypeId::BigInt),
            _ => None,
        }
    }

    pub fn as_index(self) -> u32 {
        self as u32
    }

    pub fn valid_type_names() -> &'static [&'static str] {
        &["number", "int", "float", "str", "bool", "bigint"]
    }
}
