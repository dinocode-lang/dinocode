// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/io.rs
//  Desc:       Native I/O functions for input/output operations
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_macros::dinof;
use crate::{
    runtime::context::Runtime,
    types::DinoRef,
    errors::{
        Result,
        RuntimeError,
    },
};
use dinocode_platform::io;

crate::register_module! {
    name: init_io,
    functions: [print, input]
}

#[dinof(raw)]
pub fn print(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
    let stack_slice = &runtime.memory.stack()[args_start..args_start + args_count];
    
    if args_count == 1 {
        let arg = stack_slice[0];
        let string_value = arg.to_display_string(&runtime.memory)?;
        io::println(&string_value);
    } else {
        for &arg in stack_slice {
            let string_value = arg.to_display_string(&runtime.memory)?;
            io::print(&string_value);
        }
        io::println("");
    }
    
    Ok(DinoRef::NONE)
}

#[dinof(raw)]
pub fn input(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
    let prompt = if args_count > 0 {
        let arg = runtime.memory.stack()[args_start];
        arg.to_display_string(&runtime.memory)?
    } else {
        String::new()
    };

    let input = io::input(&prompt)
        .map_err(|e| RuntimeError::ReadInputFailed(e))?;

    let trimmed_input = input.trim_end_matches(['\r', '\n']);

    Ok(runtime.memory.alloc_string(trimmed_input))
}
