use colored::Colorize;
use dinocode::compiler::parser::types::Bytecode;
use dinocode_core::{
    utils::opcode::opcode_name,
    types::opcode_defs::opcode::*,
};

pub fn display_bytecode(bytecode: &Bytecode) {
    display_constants(bytecode);
    display_functions(bytecode);
    display_instructions(bytecode);
}

fn display_constants(bytecode: &Bytecode) {
    if bytecode.const_pool.is_empty() {
        return;
    }

    println!("{}", "► CONSTANT POOL".bright_white().bold());
    println!("{}", "─".repeat(60).bright_black());

    for (idx, constant) in bytecode.const_pool.iter().enumerate() {
        let const_str = format!("{:?}", constant);
        println!(
            "  [{:<3}] {}",
            idx.to_string().bright_yellow(),
            const_str.bright_green()
        );
    }
    println!();
}

fn display_functions(bytecode: &Bytecode) {
    if bytecode.functions.is_empty() {
        return;
    }

    println!("{}", "► FUNCTIONS".bright_white().bold());
    println!("{}", "─".repeat(60).bright_black());

    for (idx, func) in bytecode.functions.iter().enumerate() {
        println!(
            "  [{:<3}] (params: {}, returns: {})",
            idx.to_string().bright_yellow(),
            func.param_count.to_string().bright_magenta(),
            func.return_count.to_string().bright_blue()
        );
        println!(
            "        IP: {}..{}",
            func.start_ip.to_string().bright_green(),
            func.end_ip.to_string().bright_green()
        );
    }
    println!();
}

fn display_instructions(bytecode: &Bytecode) {
    if bytecode.instructions.is_empty() {
        println!("{}", "No instructions found.".yellow());
        return;
    }

    println!("{}", "► INSTRUCTIONS".bright_white().bold());
    println!("{}", "─".repeat(60).bright_black());

    let instructions = &bytecode.instructions;
    let mut i = 0;

    while i < instructions.len() {
        let instruction = instructions[i];
        let opcode_byte = ((instruction >> 24) & 0xFF) as u8;
        let payload = instruction & 0x00FFFFFF;

        let op_name = opcode_name(opcode_byte);
        let operand_desc = decode_operand(opcode_byte, payload, bytecode);

        println!(
            "  [{:>04}] {:<20} {}",
            i.to_string().bright_yellow(),
            op_name.bright_cyan().bold(),
            operand_desc.bright_green()
        );

        i += 1;
    }

    println!();
    println!(
        "{}",
        format!("Total instructions: {}",
            instructions.len()
        ).bright_blue()
    );
    println!();
}

fn decode_operand(opcode: u8, operand: u32, bytecode: &Bytecode) -> String {
    match opcode {
        LOAD_CONST => {
            if (operand as usize) < bytecode.const_pool.len() {
                format!("#{} {:?}", operand, bytecode.const_pool[operand as usize])
            } else {
                format!("#{} (out of range)", operand)
            }
        }
        GET_LOCAL | SET_LOCAL => {
            format!("local[{}]", operand)
        }
        GET_GLOBAL | SET_GLOBAL => {
            format!("global[{}]", operand)
        }
        JUMP | JUMP_IF | JUMP_IF_NOT => {
            format!("-> {}", operand)
        }
        CALL => {
            format!("argc: {}", operand)
        }
        MAKE_ARRAY | MAKE_OBJECT | MAKE_CLASS => {
            format!("size: {}", operand)
        }
        _ => {
            if operand > 0 {
                format!("{}", operand)
            } else {
                String::new()
            }
        }
    }
}
