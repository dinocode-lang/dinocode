// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/bound/prototype.rs
//  Desc:       BoundFunction prototype
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    types::DinoRef,
    errors::Result,
    builtins::bound::view::BoundFunctionView
};
use dinocode_macros::{dinoclass, dinomethods, raw, symbol};

crate::register_module! {
    name: init_bound_function,
    classes: [BoundFunction]
}

#[dinoclass]
pub struct BoundFunction;

#[dinomethods]
impl BoundFunction {
    #[raw]
    #[symbol(name="call")]
    pub fn call(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        let obj = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let bound = BoundFunctionView::from_ref_unchecked(obj);
        bound.call(args_start, args_count, runtime)
    }
}
