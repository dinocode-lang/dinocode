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

use dinocode_core::{
    DinoError,
    errors::RuntimeError,
    utils::source_map::SourceMap,
};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("VM Error at IP {ip}: {source}")]
pub struct VMError {
    pub source: RuntimeError,
    pub ip: usize,
    pub traces: Vec<usize>,
}

pub type VmResult<T> = std::result::Result<T, VMError>;

impl VMError {
    pub fn to_dino_error(&self, source_map: &SourceMap) -> DinoError<'static> {
        let primary_ip = self.traces.last().copied().unwrap_or(self.ip);
        let (line, column) = source_map.get_location(primary_ip).unwrap_or((1, 1));
        let mut dino = self.source.to_dino_error(line as u32, column as u32);

        let caller_ips = if self.traces.len() > 1 {
            &self.traces[..self.traces.len() - 1]
        } else {
            &[]
        };

        for &ip in caller_ips.iter().rev() {
            let lookup_ip = ip.saturating_sub(1);
            let (f_line, f_col) = source_map.get_location(lookup_ip).unwrap_or((1, 1));
            dino = dino.add_stack_frame(f_line as u32, f_col as u32);
        }
        
        dino
    }
}

