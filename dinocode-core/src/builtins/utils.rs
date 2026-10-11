// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/utils.rs
//  Desc:       Native utility functions.
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

crate::register_module! {
    name: init_utils,
    functions: [panic]
}

#[dinof(raw)]
pub fn panic(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Panic {
            message: "execution aborted".to_string(),
            help: None,
            info: None,
        });
    }

    let (message, help, info) = {
        let msg_arg = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let msg = msg_arg.try_as_string(&mut runtime.memory)?;
        let help = if args_count >= 2 {
            let help_arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
            Some(help_arg.try_as_string(&mut runtime.memory)?)
        } else {
            None
        };
        let info = if args_count >= 3 {
            let info_arg = unsafe { runtime.memory.stack.get_unchecked(args_start + 2) };
            Some(info_arg.try_as_string(&mut runtime.memory)?)
        } else {
            None
        };
        (msg, help, info)
    };
    Err(RuntimeError::Panic { message, help, info })
}
