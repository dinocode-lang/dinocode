// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/main.rs
//  Desc:       DinoCode CLI
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

mod args;
mod display;

use args::Args;
use colored::Colorize;
use dinocode_core::DinoError;
use dinocode::{
    compiler::lexer::Lexer,
    compiler::parser::Parser,
    interpreter::VirtualMachine,
};
use std::{
    fs,
    process,
};

fn execute_code(
    code: &str,
    main_args: Vec<String>,
    show_tokens: bool,
    show_bytecode: bool,
) -> Result<(), String> {
    let tokens = match Lexer::tokenize(code) {
        Ok(tokens) => tokens,
        Err(e) => {
            let dino_error: DinoError = e.into();
            return Err(dino_error.render(code));
        }
    };

    if show_tokens {
        display::display_tokens(tokens.as_slice());
    }

    if show_bytecode || (!show_tokens && !show_bytecode) {
        let (bytecode, source_map) = match Parser::compile(tokens.as_slice(), code) {
            Ok((b, sm)) => (b, sm),
            Err(e) => {
                let dino_error: DinoError = e.into();
                return Err(dino_error.render(code));
            }
        };

        if show_bytecode {
            display::display_bytecode(&bytecode);
        }

        if !show_tokens && !show_bytecode {
            let mut vm = VirtualMachine::from_bytecode(bytecode);
            if let Err(e) = vm.run_with_args(&main_args) {
                let dino_error = e.to_dino_error(&source_map);
                return Err(dino_error.render(code));
            }
        }
    }

    Ok(())
}

fn print_version() {
    println!("{} v{}", "DinoCode".bright_cyan().bold(), env!("CARGO_PKG_VERSION"));
    println!();
    println!("Author: {}", "Ismael Quiroz (@BlassGO)".bright_white());
    println!("Copyright: (C) 2025-2026 Ismael Quiroz");
    println!("License: Apache License 2.0");
    println!("Website: https://github.com/dinocode-lang/dinocode");
    println!();
}

fn print_help_extended() {
    println!("{}", "DinoCode - Programming Language".bright_cyan().bold());
    println!();
    println!("Usage: dinocode [OPTIONS] <file.dino> [ARGS...]");
    println!();
    println!("{}", "Arguments:".bright_white().bold());
    println!("  <file.dino>    DinoCode source file to execute");
    println!("  [ARGS...]      Arguments to pass to the <file.dino>");
    println!();
    println!("{}", "Options:".bright_white().bold());
    println!("      --version      Show version information");
    println!("      --help         Show this help message");
    println!("      --tokens       Display lexer tokens with attributes");
    println!("      --bytecode     Display compiled bytecode");
    println!();
    println!("{}", "Examples:".bright_white().bold());
    println!("  dinocode script.dino");
    println!("  dinocode script.dino arg1 arg2");
    println!();
}

fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap_or(());

    let args = Args::parse();

    if args.help {
        print_help_extended();
        process::exit(0);
    }

    if args.version {
        print_version();
        process::exit(0);
    }

    dinocode::init();

    let filename = match args.file {
        Some(file) => file,
        None => {
            print_help_extended();
            process::exit(0);
        }
    };

    if let Err(e) = fs::metadata(&filename) {
        eprintln!("{}: {}", "Error".bright_red().bold(), format!("Cannot access '{}': {}", filename, e));
        process::exit(1);
    }

    let code = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}: {}", "Error".bright_red().bold(), format!("Cannot read '{}': {}", filename, e));
            process::exit(1);
        }
    };

    if let Err(e) = execute_code(&code, args.main_args, args.show_tokens, args.show_bytecode) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
