// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/interpreter/core/vm.rs
//  Desc:       Handles execution of bytecode.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{DinoRef, opcode, Instruction, UserFunction},
    builtins::{
        range::view::RangeView,
        array::view::ArrayView,
        string::view::StringView,
        object::{
            prototype::Object,
            view::ObjectView,
        },
    },
    memory::{
        MemoryManager,
        types::property_flags::{
            PROP_NORMAL,
            PROP_PROTOTYPE,
        },
    },
    utils::TypeConverter,
    errors::RuntimeError,
    builtins::io,
    runtime::context::Runtime,
};
use crate::{
    compiler::parser::types::Bytecode,
    interpreter::{
        core::execution::utils::{
            binary_ops,
            dispatch::{as_object, get_index, set_index, call},
            unary_ops,
            iterators,
            vm_helpers,
        },
        errors::{VMError, VmResult},
    },
};

pub struct VirtualMachine {
    memory: MemoryManager,
    bytecode: Vec<u32>,
    const_pool: Vec<DinoRef>,
    functions: Vec<UserFunction>,
    main_function: Option<DinoRef>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            memory: MemoryManager::new(),
            bytecode: Vec::new(),
            const_pool: Vec::new(),
            functions: Vec::new(),
            main_function: None,
        }
    }

    pub fn from_bytecode(bytecode: Bytecode) -> Self {
        let num_globals = bytecode.global_count as usize;
        let mut memory = bytecode.memory_manager;
        
        if num_globals > 0 {
            memory.stack.extend_from_slice(&vec![DinoRef::NONE; num_globals]);
        }

        memory.arena.set_globals();
        
        Self {
            memory,
            bytecode: bytecode.instructions,
            const_pool: bytecode.const_pool,
            functions: bytecode.functions,
            main_function: bytecode.main_function,
        }
    }

    #[inline(always)]
    fn decode(&self, ip: usize) -> (u8, u32) {
        let instruction = self.bytecode[ip];
        let op = ((instruction >> 24) & 0xFF) as u8;
        let payload = instruction & 0x00FFFFFF;
        (op, payload)
    }

    fn get_call_stack_ips(&self, error_ip: usize) -> Vec<usize> {
        vm_helpers::get_call_stack_ips(&self.memory, self.main_function, error_ip)
    }

    #[inline]
    pub fn run(&mut self) -> VmResult<()> {
        self.run_with_args(&[])
    }

    #[inline]
    pub fn run_with_args(&mut self, main_args: &[String]) -> VmResult<()> {
        let len = self.bytecode.len();
        
        self.run_from(0, len)?;

        if let Some(main_ref) = self.main_function {
            let main_id = main_ref.get_function_id();
            let func = &self.functions[main_id as usize];
            let start_ip = func.start_ip;
            let param_count = func.param_count;
            self.memory.stack.push(main_ref);
            if param_count > 0 {
                let array = vm_helpers::args_to_array(&mut self.memory, main_args)
                    .map_err(|e| VMError { source: e, ip: len, traces: self.get_call_stack_ips(len) })?;
                self.memory.stack.push(array);
            }

            let args_start = self.memory.stack.depth() - param_count as usize;
            
            self.memory.call_stack.push(len as u32, main_id, func, args_start, param_count as usize, 0, &mut self.memory.stack)
                .map_err(|e| VMError { source: e, ip: len, traces: vm_helpers::get_call_stack_ips(&self.memory, self.main_function, len) })?;
            
            self.run_from(start_ip, len)?;
        }
        
        Ok(())
    }

    fn run_from(&mut self, mut ip: usize, len: usize) -> VmResult<()> {
        
        macro_rules! vm_err {
            ($err:expr) => {
                Err(VMError { source: $err, ip, traces: self.get_call_stack_ips(ip) })
            };
        }

        macro_rules! try_vm {
            ($expr:expr) => {
                match $expr {
                    Ok(v) => v,
                    Err(e) => return vm_err!(e),
                }
            };
        }

        while ip < len {
            let (op, payload) = self.decode(ip);
            
            match op {
                opcode::LOAD_CONST => {
                    let value = unsafe { *self.const_pool.get_unchecked(payload as usize) };
                    self.memory.stack.push(value);
                },

                opcode::TRUE => {
                    self.memory.stack.push(DinoRef::TRUE);
                },
                
                opcode::FALSE => {
                    self.memory.stack.push(DinoRef::FALSE);
                },
                
                opcode::NONE => {
                    self.memory.stack.push(DinoRef::NONE);
                },

                opcode::GET_LOCAL => {
                    let var_idx = payload as u32;
                    let value = unsafe { self.memory.stack.get_local_unchecked(var_idx) };
                    self.memory.stack.push(value);
                },

                opcode::GET_GLOBAL => {
                    let var_idx = payload as u32;
                    let value = unsafe { self.memory.stack.get_unchecked(var_idx as usize) };
                    self.memory.stack.push(value);
                },

                opcode::SET_LOCAL => {
                    let var_idx = payload as u32;
                    unsafe {
                        let value = self.memory.stack.peek_unchecked(0);
                        self.memory.stack.set_local_unchecked(var_idx, value);
                    }
                },

                opcode::SET_GLOBAL => {
                    let var_idx = payload as u32;
                    unsafe {
                        let value = self.memory.stack.peek_unchecked(0);
                        self.memory.stack.set_unchecked(var_idx as usize, value);
                    }
                },

                opcode::DROP_LOCAL => {
                    let var_idx = payload as u32;
                    unsafe { self.memory.stack.set_local_unchecked(var_idx, DinoRef::NONE); }
                },

                opcode::DROP_GLOBAL => {
                    let var_idx = payload as u32;
                    unsafe { self.memory.stack.set_unchecked(var_idx as usize, DinoRef::NONE); }
                },
                
                opcode::POP => {
                    unsafe { self.memory.stack.pop_unchecked(); }
                },
                
                opcode::POP_N => {
                    let count = payload as usize;
                    unsafe { self.memory.stack.pop_n_unchecked(count); }
                },
                
                opcode::DUP => {
                    let offset = payload as usize;
                    let depth = self.memory.stack.depth();
                    let value = unsafe { self.memory.stack.get_unchecked(depth - 1 - offset) };
                    self.memory.stack.push(value);
                },
                
                opcode::ADD | opcode::SUB | opcode::MUL | opcode::DIV | opcode::FLOOR_DIV | opcode::MOD | opcode::POW |
                opcode::EQ | opcode::NE | opcode::GT | opcode::LT | opcode::GE | opcode::LE |
                opcode::BIT_AND | opcode::BIT_OR | opcode::BIT_XOR |
                opcode::DOT | opcode::IN => {
                    ip = try_vm!(binary_ops::binary_op(op, &mut self.memory, &self.functions, ip));
                },
                
                opcode::NOT => {
                    try_vm!(unary_ops::unary_not(&mut self.memory));
                },
                
                opcode::NEG => {
                    try_vm!(unary_ops::unary_neg(&mut self.memory));
                },
                
                opcode::JUMP => {
                    ip = payload as usize;
                    continue;
                },
                
                opcode::JUMP_IF => {
                    let condition = unsafe { self.memory.stack.peek_unchecked(0) };
                    let truthy = try_vm!(condition.try_as_bool(&mut self.memory));
                    
                    unsafe { self.memory.stack.pop_unchecked(); }
                    
                    if truthy {
                        ip = payload as usize;
                        continue;
                    }
                },
                
                opcode::JUMP_IF_NOT => {
                    let condition = unsafe { self.memory.stack.peek_unchecked(0) };
                    let truthy = try_vm!(condition.try_as_bool(&mut self.memory));
                    
                    unsafe { self.memory.stack.pop_unchecked(); }
                    
                    if !truthy {
                        ip = payload as usize;
                        continue;
                    }
                },
                
                opcode::FOR_INIT => {
                    let var_idx = payload as u32;
                    let next_opcode = try_vm!(iterators::for_init(&mut self.memory, var_idx));
                    
                    let next_ip = ip + 1;
                    if next_ip < len {
                        let next_inst = Instruction(self.bytecode[next_ip]);
                        self.bytecode[next_ip] = Instruction::new_raw(next_opcode, next_inst.payload()).0;
                    }
                },

                opcode::FOR_ITER => {
                    return vm_err!(RuntimeError::InternalError("unpatched FOR_ITER instruction reached"));
                },
                
                opcode::FOR_ITER_ARRAY => {
                    let var_idx = payload as u32;
                    let has_more = try_vm!(iterators::for_iter_array(&mut self.memory, var_idx));
                    if has_more {
                        ip += 2;    // Skip JUMP_IF_NOT
                        continue;
                    }
                },
                
                opcode::FOR_ITER_RANGE => {
                    let var_idx = payload as u32;
                    let has_more = try_vm!(iterators::for_iter_range(&mut self.memory, var_idx));
                    if has_more {
                        ip += 2;    // Skip JUMP_IF_NOT
                        continue;
                    }
                },
                
                opcode::FOR_ITER_STRING => {
                    let var_idx = payload as u32;
                    let has_more = try_vm!(iterators::for_iter_string(&mut self.memory, var_idx));
                    if has_more {
                        ip += 2;    // Skip JUMP_IF_NOT
                        continue;
                    }
                },
                
                opcode::FOR_DROP => {
                    let var_idx = payload as u32;
                    // var_idx: target
                    // var_idx + 1: current index
                    // var_idx + 2: limit/count
                    // var_idx + 3: step
                    unsafe {
                        self.memory.stack.set_local_unchecked(var_idx, DinoRef::NONE);
                        self.memory.stack.set_local_unchecked(var_idx + 1, DinoRef::NONE);
                        self.memory.stack.set_local_unchecked(var_idx + 2, DinoRef::NONE);
                        self.memory.stack.set_local_unchecked(var_idx + 3, DinoRef::NONE);
                    }
                },
                
                opcode::RETURN => {
                    let return_address = self.memory.call_stack.pop(&mut self.memory.stack);
                    ip = return_address as usize;
                    continue;
                },
                
                opcode::RETURN_REF => {
                    let return_address = self.memory.call_stack.pop_with_ref(&mut self.memory.stack);
                    ip = return_address as usize;
                    continue;
                },
                
                opcode::RETURN_SELF => {
                    let return_address = self.memory.call_stack.pop_with_self(&mut self.memory.stack);
                    ip = return_address as usize;
                    continue;
                },

                opcode::MAKE_ARRAY => {
                    let count = payload as usize;
                    let array = try_vm!(ArrayView::new_from_stack(count, &mut self.memory));
                    unsafe {
                        self.memory.stack.pop_n_unchecked(count);
                        self.memory.stack.push_unchecked(array.as_ref());
                    }
                },
                
                opcode::MAKE_OBJECT => {
                    let count = payload as usize;
                    let proto = Object::as_ref(&mut self.memory);
                    let object = try_vm!(ObjectView::new_from_stack(proto, count, PROP_NORMAL, &mut self.memory));
                    unsafe {
                        self.memory.stack.pop_n_unchecked(count);
                        self.memory.stack.push_unchecked(object.as_ref());
                    }
                },

                opcode::MAKE_CLASS => {
                    let count = payload as usize * 2;
                    let depth = self.memory.stack.depth();
                    let proto = unsafe { self.memory.stack.get_unchecked(depth - count - 1) };
                    let object = try_vm!(ObjectView::new_from_stack(proto, count, PROP_PROTOTYPE, &mut self.memory));
                    unsafe {
                        self.memory.stack.pop_n_unchecked(count + 1);
                        self.memory.stack.push_unchecked(object.as_class_ref());
                    }
                },
                
                opcode::MAKE_RANGE => {
                    let inclusive = payload == 1;
                    
                    let end_ref = unsafe { self.memory.stack.peek_unchecked(0) };
                    let start_ref = unsafe { self.memory.stack.peek_unchecked(1) };
                    
                    let start = try_vm!(start_ref.try_as_int(&mut self.memory));
                    let end = try_vm!(end_ref.try_as_int(&mut self.memory));
                    
                    let range = try_vm!(RangeView::new(start, end, 1, inclusive, &mut self.memory));

                    unsafe {
                        self.memory.stack.pop_n_unchecked(2);
                        self.memory.stack.push_unchecked(range.as_ref());
                    }
                },

                opcode::GET_MEMBER => {
                    let key = unsafe { self.memory.stack.peek_unchecked(0) };
                    let obj = unsafe { self.memory.stack.peek_unchecked(1) };
                    let object = try_vm!(as_object(obj, &mut self.memory));
                    let result = try_vm!(object.get(key, &mut self.memory));

                    unsafe {
                        self.memory.stack.pop_n_unchecked(2);
                        self.memory.stack.push_unchecked(result);
                    }
                },

                opcode::GET_MEMBER_PREP => {
                    let key = unsafe { self.memory.stack.peek_unchecked(0) };
                    let obj = unsafe { self.memory.stack.peek_unchecked(1) };
                    let object = try_vm!(as_object(obj, &mut self.memory));
                    let result = try_vm!(object.get(key, &mut self.memory));
                    self.memory.stack.push(result);
                },

                opcode::GET_INDEX => {
                    let index = unsafe { self.memory.stack.peek_unchecked(0) };
                    let object = unsafe { self.memory.stack.peek_unchecked(1) };
                    let result = try_vm!(get_index(object, index, &mut self.memory));

                    unsafe {
                        self.memory.stack.pop_n_unchecked(2);
                        self.memory.stack.push_unchecked(result);
                    }
                },

                opcode::GET_INDEX_PREP => {
                    let index = unsafe { self.memory.stack.peek_unchecked(0) };
                    let object = unsafe { self.memory.stack.peek_unchecked(1) };
                    let result = try_vm!(get_index(object, index, &mut self.memory));
                    
                    self.memory.stack.push(result);
                },
                
                opcode::GET_METHOD => {
                    let obj = unsafe { self.memory.stack.peek_unchecked(1) };
                    let key = unsafe { self.memory.stack.peek_unchecked(0) };
                    let object = try_vm!(as_object(obj, &mut self.memory));
                    let method = try_vm!(object.get(key, &mut self.memory));
                    
                    unsafe {
                        let args_start = self.memory.stack.depth() - 2;
                        self.memory.stack.set_unchecked(args_start, method);
                        self.memory.stack.set_unchecked(args_start + 1, obj);
                    }
                },
                
                opcode::GET_MEMBER_BOUND => {
                    let obj = unsafe { self.memory.stack.peek_unchecked(1) };
                    let object = try_vm!(as_object(obj, &mut self.memory));
                    let depth = self.memory.stack.depth();
                    let start = depth - 2;
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    try_vm!(object.get_attr(start, &mut runtime));
                },
                
                opcode::SET_MEMBER => {
                    let value = unsafe { self.memory.stack.peek_unchecked(0) };
                    let key = unsafe { self.memory.stack.peek_unchecked(1) };
                    let obj = unsafe { self.memory.stack.peek_unchecked(2) };
                    let object = try_vm!(as_object(obj, &mut self.memory));
                    try_vm!(object.set(key, value, PROP_NORMAL, &mut self.memory));
                    unsafe {
                        self.memory.stack.pop_n_unchecked(3);
                        self.memory.stack.push_unchecked(value);
                    }
                },

                opcode::SET_MEMBER_PREP => {
                    let value = unsafe { self.memory.stack.peek_unchecked(0) };
                    let key = unsafe { self.memory.stack.peek_unchecked(1) };
                    let obj = unsafe { self.memory.stack.peek_unchecked(2) };
                    let object = try_vm!(as_object(obj, &mut self.memory));
                    try_vm!(object.set(key, value, PROP_NORMAL, &mut self.memory));
                    unsafe {
                        self.memory.stack.pop_n_unchecked(3);
                        self.memory.stack.push_unchecked(value);
                    }
                },
                
                opcode::SET_INDEX => {
                    let value = unsafe { self.memory.stack.peek_unchecked(0) };
                    let index = unsafe { self.memory.stack.peek_unchecked(1) };
                    let object = unsafe { self.memory.stack.peek_unchecked(2) };
                    let result = try_vm!(set_index(object, index, value, &mut self.memory));
                    unsafe {
                        self.memory.stack.pop_n_unchecked(3);
                        self.memory.stack.push_unchecked(result);
                    }
                },

                opcode::SET_INDEX_PREP => {
                    let value = unsafe { self.memory.stack.peek_unchecked(0) };
                    let index = unsafe { self.memory.stack.peek_unchecked(1) };
                    let object = unsafe { self.memory.stack.peek_unchecked(2) };
                    let result = try_vm!(set_index(object, index, value, &mut self.memory));
                    unsafe {
                        self.memory.stack.pop_n_unchecked(3);
                        self.memory.stack.push_unchecked(result);
                    }
                },

                opcode::INPUT => {
                    let depth = self.memory.stack.depth();
                    let args_start = depth - 1;
                    let result = {
                        let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                        try_vm!(io::input(&mut runtime, args_start, 1))
                    };
                    unsafe {
                        self.memory.stack.pop_n_unchecked(2); // Expected as binary operation
                        self.memory.stack.push_unchecked(result);
                    }
                },

                opcode::CALL => {
                    let count = payload as usize;
                    let depth = self.memory.stack.depth();
                    let args_start = depth - count;
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    try_vm!(call(&mut runtime, args_start, count));
                },

                opcode::STR_BUILD => {
                    let count = payload as usize;
                    let depth = self.memory.stack.depth();
                    let start = depth - count;
                    
                    let mut result_string = String::new();
                    for i in start..depth {
                        let element = unsafe { self.memory.stack.get_unchecked(i) };
                        let s = try_vm!(element.try_as_string(&mut self.memory));
                        result_string.push_str(&s);
                    }
                    
                    let string = StringView::new(&result_string, &mut self.memory);
                    
                    unsafe {
                        self.memory.stack.pop_n_unchecked(count);
                        self.memory.stack.push_unchecked(string.as_ref());
                    }
                },
                
                opcode::TO => {
                    let value = unsafe { self.memory.stack.peek_unchecked(0) };
                    let target_type = payload;
                    
                    let converted = match target_type {
                        0 => try_vm!(TypeConverter::to_number(value, &mut self.memory)),    // number
                        1 => try_vm!(TypeConverter::to_int(value, &mut self.memory)),      // int
                        2 => try_vm!(TypeConverter::to_float(value, &mut self.memory)),    // float
                        3 => try_vm!(TypeConverter::to_string(value, &mut self.memory)),   // string
                        4 => try_vm!(TypeConverter::to_bool(value, &mut self.memory)),     // bool
                        5 => try_vm!(TypeConverter::to_bigint(value, &mut self.memory)),  // bigint
                        _ => return vm_err!(RuntimeError::InvalidTypeIndex),
                    };
                    
                    unsafe {
                        self.memory.stack.pop_unchecked();
                        self.memory.stack.push_unchecked(converted);
                    }
                },
                
                _ => return vm_err!(RuntimeError::UnknownOpCode(op)),
            }

            #[cfg(target_arch = "wasm32")]
            dinocode_platform::thread::step(ip);

            ip += 1;
        }
        
        Ok(())
    }
}
