// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/analysis/analyzer.rs
//  Desc:       Bytecode analyzer for WASM
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use wasm_bindgen::prelude::*;
use console_error_panic_hook::set_once;
use colored::control;
use crate::bytecode::BytecodeInfo;
use dinocode::{
    compiler::lexer::Lexer,
    compiler::parser::Parser,
};
use crate::analysis::cfg::build_cfg;

#[wasm_bindgen]
pub struct BytecodeAnalyzer {
    info: BytecodeInfo,
    source: String,
}

#[wasm_bindgen]
impl BytecodeAnalyzer {
    #[wasm_bindgen(constructor)]
    pub fn from_source(source: &str) -> Result<BytecodeAnalyzer, JsValue> {
        set_once();
        control::set_override(true);
        dinocode::init();

        let tokens = match Lexer::tokenize(source) {
            Ok(tokens) => tokens,
            Err(e) => {
                return Err(JsValue::from_str(&e.to_string()));
            }
        };

        let (mut bytecode, source_map) = match Parser::compile(tokens.iter().as_slice(), source) {
            Ok((b, sm)) => (b, sm),
            Err(e) => {
                return Err(JsValue::from_str(&e.to_string()));
            }
        };

        let info = BytecodeInfo::from_bytecode_and_source_map(&mut bytecode, &source_map);
        
        Ok(BytecodeAnalyzer { info, source: source.to_string() })
    }

    #[wasm_bindgen(js_name = getFullInfo)]
    pub fn get_full_info(&self) -> Result<String, JsValue> {
        self.info.to_json()
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    #[wasm_bindgen(js_name = getConstants)]
    pub fn get_constants(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.info.constants)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    #[wasm_bindgen(js_name = getFunctions)]
    pub fn get_functions(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.info.functions)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    #[wasm_bindgen(js_name = getInstructions)]
    pub fn get_instructions(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.info.instructions)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    #[wasm_bindgen(js_name = getInstructionsAtLine)]
    pub fn get_instructions_at_line(&self, line: u32) -> Result<String, JsValue> {
        let instructions = self.info.get_instructions_at_line(line);
        serde_json::to_string_pretty(&instructions)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }

    #[wasm_bindgen(js_name = getInstructionAt)]
    pub fn get_instruction_at(&self, ip: u32) -> Result<String, JsValue> {
        match self.info.get_instruction_at(ip) {
            Some(instruction) => {
                serde_json::to_string_pretty(instruction)
                    .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
            }
            None => Err(JsValue::from_str(&format!("Instruction at IP {} not found", ip)))
        }
    }

    #[wasm_bindgen(js_name = getFunctionRange)]
    pub fn get_function_range(&self, function_index: u32) -> Result<String, JsValue> {
        match self.info.get_function_range(function_index) {
            Some((start, end)) => {
                let range = format!("{{\"start\": {}, \"end\": {}}}", start, end);
                Ok(range)
            }
            None => Err(JsValue::from_str(&format!("Function at index {} not found", function_index)))
        }
    }

    #[wasm_bindgen(js_name = getControlFlowGraph)]
    pub fn get_control_flow_graph(&self) -> Result<String, JsValue> {
        let cfg = build_cfg(&self.info, &self.source);
        serde_json::to_string_pretty(&cfg)
            .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
    }
}
