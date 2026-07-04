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
    memory::MemoryManager,
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
pub fn panic(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Panic {
            message: "execution aborted".to_string(),
            help: None,
            info: None,
        });
    }

    let (message, help, info) = {
        let msg_arg = memory.stack().get(args_start).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        let msg = msg_arg.try_as_string(memory)
            .unwrap_or_else(|_| msg_arg.to_string());

        let help = if args_count >= 2 {
            let help_arg = memory.stack().get(args_start + 1).copied()
                .ok_or(RuntimeError::StackUnderflow)?;
            Some(help_arg.try_as_string(memory)
                .unwrap_or_else(|_| help_arg.to_string()))
        } else {
            None
        };

        let info = if args_count >= 3 {
            let info_arg = memory.stack().get(args_start + 2).copied()
                .ok_or(RuntimeError::StackUnderflow)?;
            Some(info_arg.try_as_string(memory)
                .unwrap_or_else(|_| info_arg.to_string()))
        } else {
            None
        };

        (msg, help, info)
    };

    Err(RuntimeError::Panic { message, help, info })
}
