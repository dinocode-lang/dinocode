use wasm_bindgen::prelude::*;
use console_error_panic_hook::set_once;
use colored::control;
use dinocode::{
    compiler::lexer::Lexer,
    compiler::parser::Parser,
    interpreter::VirtualMachine,
};
use dinocode_core::DinoError;
use dinocode_platform::io;

mod bytecode;
mod analysis;

pub use bytecode::{BytecodeInfo, ConstantInfo, FunctionInfo, InstructionInfo};
pub use analysis::BytecodeAnalyzer;

#[wasm_bindgen]
pub struct DinoWasm;

#[wasm_bindgen]
pub struct ExecutionError {
    message: String,
    pub line: u32,
    pub col: u32,
}

#[wasm_bindgen]
impl ExecutionError {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String { self.message.clone() }
}

#[wasm_bindgen]
pub struct ExecutionResult {
    success: bool,
    error: Option<ExecutionError>,
}

#[wasm_bindgen]
impl ExecutionResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool { self.success }

    #[wasm_bindgen(getter)]
    pub fn error(self) -> Option<ExecutionError> { self.error }
}

#[wasm_bindgen]
impl DinoWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_once();
        control::set_override(true);
        dinocode::init();
        Self
    }

    #[wasm_bindgen(js_name = setPrintCallback)]
    pub fn set_print_callback(&self, f: js_sys::Function) {
        io::set_print_hook(f);
    }

    #[wasm_bindgen(js_name = setInputCallback)]
    pub fn set_input_callback(&self, f: js_sys::Function) {
        io::set_input_hook(f);
    }

    #[wasm_bindgen(js_name = setSleepCallback)]
    pub fn set_sleep_callback(&self, f: js_sys::Function) {
        dinocode_platform::thread::set_sleep_hook(f);
    }

    #[wasm_bindgen(js_name = setStepCallback)]
    pub fn set_step_callback(&self, f: js_sys::Function) {
        dinocode_platform::thread::set_step_hook(f);
    }

    #[wasm_bindgen(js_name = enableStep)]
    pub fn enable_step(&self) { dinocode_platform::thread::enable_step(); }

    #[wasm_bindgen(js_name = disableStep)]
    pub fn disable_step(&self) { dinocode_platform::thread::disable_step(); }

    #[wasm_bindgen(js_name = analyzeBytecode)]
    pub fn analyze_bytecode(&self, source: &str) -> Result<BytecodeAnalyzer, JsValue> {
        BytecodeAnalyzer::from_source(source)
    }

    #[wasm_bindgen(js_name = executeCode)]
    pub fn execute_code(&self, source: &str) -> ExecutionResult {
        self.execute_code_with_args(source, Vec::new())
    }

    #[wasm_bindgen(js_name = executeCodeWithArgs)]
    pub fn execute_code_with_args(&self, source: &str, args: Vec<String>) -> ExecutionResult {
        let tokens = match Lexer::tokenize(source) {
            Ok(tokens) => tokens,
            Err(e) => {
                let err: DinoError = e.into();
                return ExecutionResult {
                    success: false,
                    error: Some(ExecutionError {
                        message: err.render(source),
                        line: err.line,
                        col: err.column,
                    }),
                };
            }
        };

        let (bytecode, source_map) = match Parser::compile(tokens.iter().as_slice(), source) {
            Ok((b, sm)) => (b, sm),
            Err(e) => {
                let err: DinoError = e.into();
                return ExecutionResult {
                    success: false,
                    error: Some(ExecutionError {
                        message: err.render(source),
                        line: err.line,
                        col: err.column,
                    }),
                };
            }
        };

        let mut vm = VirtualMachine::from_bytecode(bytecode);

        match vm.run_with_args(&args) {
            Ok(_) => ExecutionResult {
                success: true,
                error: None,
            },
            Err(vm_error) => {
                let dino_error = vm_error.to_dino_error(&source_map);
                let pretty_error = dino_error.render(source);

                let (line, col) = source_map.get_location(vm_error.ip).unwrap_or((0, 0));
                
                ExecutionResult {
                    success: false,
                    error: Some(ExecutionError {
                        message: pretty_error,
                        line: line as u32,
                        col: col as u32,
                    }),
                }
            }
        }
    }
}
