// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/function/prototype.rs
//  Desc:       Function prototype
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    types::DinoRef,
    errors::{Result, RuntimeError},
    builtins::bound::view::BoundFunctionView,
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
};

crate::register_module! {
    name: init_function,
    classes: [Function]
}

#[dinoclass]
pub struct Function;

#[dinomethods]
impl Function {
    #[raw]
    pub fn bind(runtime: &mut Runtime, _args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "bind", expected: 1, got: args_count - 1 });
        }
        let bound_fn = BoundFunctionView::new_from_stack(args_count, &mut runtime.memory)?;
        Ok(bound_fn.as_ref())
    }
}
