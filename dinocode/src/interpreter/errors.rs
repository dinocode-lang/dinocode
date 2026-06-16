// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/interpreter/core/errors.rs
//  Desc:       Specific VM error wrappers.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::errors::RuntimeError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("VM Error at IP {ip}: {source}")]
pub struct VMError {
    pub source: RuntimeError,
    pub ip: usize,
    pub traces: Vec<usize>,
}

pub type VmResult<T> = std::result::Result<T, VMError>;
