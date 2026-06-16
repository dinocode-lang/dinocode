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
    types::{DinoRef, value_type, opcode, Instruction, UserFunction},
    prototypes::{
        range::Range,
        array::Array,
        object::Object,
    },
    memory::{
        MemoryManager,
        types::object_types,
    },
    utils::TypeConverter,
    errors::{RuntimeError, RuntimeErrorType},
    builtins::io,
};
use crate::{
    compiler::parser::types::Bytecode,
    interpreter::{
        core::execution::utils::binary_ops,
        core::Runtime,
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
            memory.stack_extend_from_slice(&vec![DinoRef::NONE; num_globals]);
        }

        memory.set_globals();
        
        Self {
            memory,
            bytecode: bytecode.instructions,
            const_pool: bytecode.const_pool,
            functions: bytecode.functions,
            main_function: bytecode.main_function,
        }
    }


    fn get_call_stack_ips(&self, error_ip: usize) -> Vec<usize> {
        let mut exclude_ids = Vec::new();
        
        if let Some(main_ref) = self.main_function {
            exclude_ids.push(main_ref.get_function_id());
        }
        
        let mut ips: Vec<usize> = self.memory.call_stack()
            .iter()
            .rev()
            .filter(|frame| !exclude_ids.contains(&frame.function_id))
            .take(9)
            .map(|frame| frame.return_address as usize)
            .collect();
        
        ips.reverse();
        ips.push(error_ip);
        
        ips
    }

    #[inline(always)]
    fn decode(&self, ip: usize) -> (u8, u32) {
        let instruction = self.bytecode[ip];
        let op = ((instruction >> 24) & 0xFF) as u8;
        let payload = instruction & 0x00FFFFFF;
        (op, payload)
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
            self.memory.stack_push(main_ref);
            if param_count > 0 {
                let dinoref_args: Vec<DinoRef> = main_args.iter()
                    .map(|arg| self.memory.alloc_string(arg))
                    .collect();
                
                let array_ref = Array::create_from_slice(&mut self.memory, &dinoref_args);
                self.memory.stack_push(array_ref);
            }

            let args_start = self.memory.stack_depth() - param_count as usize;
            
            self.memory.push_call_frame(len as u32, main_id, func, args_start, param_count as usize)
                .map_err(|e| VMError { 
                    source: e, 
                    ip: len, 
                    traces: self.get_call_stack_ips(len) 
                })?;
            
            self.run_from(start_ip, len)?;
        }
        
        Ok(())
    }

    fn run_from(&mut self, mut ip: usize, len: usize) -> VmResult<()> {
        
        macro_rules! vm_err {
            ($err:expr) => {
                Err(VMError { source: $err.into(), ip, traces: self.get_call_stack_ips(ip) })
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
                    if let Some(&dinoref) = self.const_pool.get(payload as usize) {
                        self.memory.stack_push(dinoref);
                    } else {
                        return vm_err!(RuntimeError::ReferenceError(format!(
                            "Constant index out of bounds: {}", payload
                        )));
                    }
                },

                opcode::TRUE => {
                    self.memory.stack_push(DinoRef::TRUE);
                },
                
                opcode::FALSE => {
                    self.memory.stack_push(DinoRef::FALSE);
                },
                
                opcode::NONE => {
                    self.memory.stack_push(DinoRef::NONE);
                },

                opcode::GET_LOCAL => {
                    let var_idx = payload as u32;
                    let value = unsafe { self.memory.get_local_variable_unchecked(var_idx) };
                    self.memory.stack_push(value);
                },

                opcode::GET_GLOBAL => {
                    let var_idx = payload as u32;
                    let value = unsafe { self.memory.get_global_variable_unchecked(var_idx) };
                    self.memory.stack_push(value);
                },

                opcode::SET_LOCAL => {
                    let var_idx = payload as u32;
                    let value = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    unsafe { self.memory.set_local_variable_unchecked(var_idx, value); }
                    
                    self.memory.stack_pop_n(1);
                    self.memory.stack_push(value);
                },

                opcode::SET_GLOBAL => {
                    let var_idx = payload as u32;
                    let value = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    unsafe { self.memory.set_global_variable_unchecked(var_idx, value); }
                    
                    self.memory.stack_pop_n(1);
                    self.memory.stack_push(value);
                },

                opcode::DROP_LOCAL => {
                    let var_idx = payload as u32;
                    unsafe { self.memory.set_local_variable_unchecked(var_idx, DinoRef::NONE); }
                },

                opcode::DROP_GLOBAL => {
                    let var_idx = payload as u32;
                    unsafe { self.memory.set_global_variable_unchecked(var_idx, DinoRef::NONE); }
                },
                
                opcode::POP => {
                    self.memory.stack_pop_n(1);
                },
                
                opcode::POP_N => {
                    let count = payload as usize;
                    self.memory.stack_pop_n(count);
                },
                
                opcode::DUP => {
                    let offset = payload as usize;
                    if offset >= self.memory.stack_depth() {
                        return vm_err!(RuntimeError::StackUnderflow);
                    }
                    let value = self.memory.stack()[self.memory.stack_depth() - 1 - offset];
                    self.memory.stack_push(value);
                },
                
                opcode::MAKE_ARRAY => {
                    let count = payload as usize;
                    if count > self.memory.stack_depth() {
                        return vm_err!(RuntimeError::StackUnderflow);
                    }

                    let start = self.memory.stack_depth() - count;
                    let array_ref = Array::create_instance(&mut self.memory, start, count);

                    self.memory.stack_pop_n(count);
                    self.memory.stack_push(array_ref);
                },
                
                opcode::STR_BUILD => {
                    let count = payload as usize;
                    if count > self.memory.stack_depth() {
                        return vm_err!(RuntimeError::StackUnderflow);
                    }
                    
                    let start = self.memory.stack_depth() - count;
                    
                    let mut result_string = String::new();
                    let elements = self.memory.stack()[start..].to_vec();
                    for element in elements {
                        let s = try_vm!(element.try_as_string(&mut self.memory));
                        result_string.push_str(&s);
                    }
                    
                    let string_ref = self.memory.alloc_string(&result_string);
                    
                    self.memory.stack_pop_n(count);
                    self.memory.stack_push(string_ref);
                },
                
                opcode::ADD | opcode::SUB | opcode::MUL | opcode::DIV | opcode::FLOOR_DIV | opcode::MOD | opcode::POW |
                opcode::EQ | opcode::NE | opcode::GT | opcode::LT | opcode::GE | opcode::LE |
                opcode::BIT_AND | opcode::BIT_OR | opcode::BIT_XOR |
                opcode::DOT | opcode::IN => {
                    let b = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let a = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let result = {
                        let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                        try_vm!(binary_ops::execute_binary_operator(a, b, op, &mut runtime))
                    };
                    
                    self.memory.stack_pop_n(2);
                    self.memory.stack_push(result);
                },
                
                opcode::NOT => {
                    let value = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    let truthy = try_vm!(value.try_as_bool(&mut self.memory));
                    
                    self.memory.stack_pop_n(1);
                    self.memory.stack_push(if truthy { DinoRef::FALSE } else { DinoRef::TRUE });
                },
                
                opcode::NEG => {
                    let value = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    let result = match value.decode_type() {
                        value_type::INT => {
                            DinoRef::int(-value.as_int())
                        },
                        value_type::FLOAT => {
                            let f_val = value.as_float();
                            DinoRef::float(-f_val)
                        },
                        _ => return vm_err!(RuntimeError::Typed(RuntimeErrorType::NegationRequiresNumber)),
                    };
                    
                    self.memory.stack_pop_n(1);
                    self.memory.stack_push(result);
                },
                
                opcode::JUMP => {
                    ip = payload as usize;
                    continue;
                },
                
                opcode::JUMP_IF => {
                    let condition = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    let truthy = try_vm!(condition.try_as_bool(&mut self.memory));
                    
                    self.memory.stack_pop_n(1);
                    
                    if truthy {
                        ip = payload as usize;
                        continue;
                    }
                },
                
                opcode::JUMP_IF_NOT => {
                    let condition = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    let truthy = try_vm!(condition.try_as_bool(&mut self.memory));
                    
                    self.memory.stack_pop_n(1);
                    
                    if !truthy {
                        ip = payload as usize;
                        continue;
                    }
                },
                
                opcode::FOR_INIT => {
                    let var_idx = payload as u32;
                    let iterable = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    self.memory.stack_pop_n(1);
                    
                    let (target, index, count, step): (DinoRef, DinoRef, DinoRef, DinoRef);
                    let opcode: u8;
                    
                    match iterable.decode_type() {
                        value_type::ARRAY => {
                            opcode = opcode::FOR_ITER_ARRAY;
                            target = iterable;
                            let len = self.memory.get_array_len(iterable.decode_index());
                            index = DinoRef::ZERO;
                            count = DinoRef::int(len as i64);
                            step = DinoRef::ONE;
                        },
                        value_type::STRING => {
                            opcode = opcode::FOR_ITER_STRING;
                            target = iterable;
                            let len = self.memory.get_const_len(iterable.decode_index());
                            index = DinoRef::ZERO;
                            count = DinoRef::int(len as i64);
                            step = DinoRef::ONE;
                        },
                        value_type::OBJECT => {
                            opcode = opcode::FOR_ITER_RANGE;
                            let handle = iterable.get_object_id();
                            if self.memory.has_object_type(handle, object_types::RANGE) {
                                index = self.memory.get_property(handle, Range::START()).unwrap_or(DinoRef::ZERO);
                                count = self.memory.get_property(handle, Range::STOP()).unwrap_or(DinoRef::ZERO);
                                step  = self.memory.get_property(handle, Range::STEP_VAL()).unwrap_or(DinoRef::ZERO);
                                target = DinoRef::NONE;
                            } else {
                                return vm_err!(RuntimeError::Typed(RuntimeErrorType::PlainObjectNotIterable));
                            }
                        },
                        _ => {
                            return vm_err!(RuntimeError::Typed(RuntimeErrorType::NotIterable(iterable.type_name().to_string())));
                        }
                    }
                    
                    unsafe {
                        self.memory.set_variable_unchecked(var_idx, target);
                        self.memory.set_variable_unchecked(var_idx + 1, index);
                        self.memory.set_variable_unchecked(var_idx + 2, count);
                        self.memory.set_variable_unchecked(var_idx + 3, step);
                    }
                    
                    let next_ip = ip + 1;
                    if next_ip < len {
                        let next_inst = Instruction(self.bytecode[next_ip]);
                        self.bytecode[next_ip] = Instruction::new_raw(opcode, next_inst.payload()).0;
                    }
                },
                opcode::FOR_ITER => {
                    return vm_err!(RuntimeError::InternalError("Unpatched FOR_ITER instruction reached.".into()));
                },
                
                opcode::FOR_ITER_ARRAY => {
                    let var_idx = payload as u32;
                    let index = unsafe { self.memory.get_variable_unchecked(var_idx + 1) };
                    let count = unsafe { self.memory.get_variable_unchecked(var_idx + 2) };
                    
                    let idx_val = index.as_int();
                    let cnt_val = count.as_int();
                    
                    if idx_val >= cnt_val {
                        self.memory.stack_push(DinoRef::FALSE);
                    } else {
                        let target = unsafe { self.memory.get_variable_unchecked(var_idx) };
                        let elem = self.memory.get_array_element(target.decode_index(), idx_val as u32);
                        
                        let new_index = DinoRef::int(idx_val.wrapping_add(1));
                        unsafe { self.memory.set_variable_unchecked(var_idx + 1, new_index); }
                        self.memory.stack_push(elem);
                        ip += 2;    // Skip JUMP_IF_NOT
                        continue;
                    }
                },
                
                opcode::FOR_ITER_RANGE => {
                    let var_idx = payload as u32;
                    let mut index = unsafe { self.memory.get_variable_unchecked(var_idx + 1) };
                    let count = unsafe { self.memory.get_variable_unchecked(var_idx + 2) };
                    let step = unsafe { self.memory.get_variable_unchecked(var_idx + 3) };
                    
                    let idx_val = index.as_int();
                    let cnt_val = count.as_int();
                    let stp_val = step.as_int();
                    let exhausted = if stp_val > 0 { idx_val >= cnt_val } else { idx_val <= cnt_val };
                    
                    if exhausted {
                        self.memory.stack_push(DinoRef::FALSE);
                    } else {
                        index = DinoRef::int(idx_val.wrapping_add(stp_val));
                        unsafe { self.memory.set_variable_unchecked(var_idx + 1, index); }
                        self.memory.stack_push(DinoRef::int(idx_val));
                        ip += 2;    // Skip JUMP_IF_NOT
                        continue;
                    }
                },
                
                opcode::FOR_ITER_STRING => {
                    let var_idx = payload as u32;
                    let index = unsafe { self.memory.get_variable_unchecked(var_idx + 1) };
                    let count = unsafe { self.memory.get_variable_unchecked(var_idx + 2) };
                    
                    let idx_val = index.as_int() as usize;
                    let cnt_val = count.as_int() as usize;
                    
                    if idx_val >= cnt_val {
                        self.memory.stack_push(DinoRef::FALSE);
                    } else {
                        let target = unsafe { self.memory.get_variable_unchecked(var_idx) };
                        let bytes = self.memory.get_const_bytes(target.decode_index());
                        
                        if idx_val < bytes.len() {
                            let s_slice = std::str::from_utf8(&bytes[idx_val..]).unwrap_or("");
                            if let Some(ch) = s_slice.chars().next() {
                                let char_len = ch.len_utf8();
                                
                                let new_index = DinoRef::int((idx_val + char_len) as i64);
                                unsafe { self.memory.set_variable_unchecked(var_idx + 1, new_index); }
                                
                                let char_str = ch.to_string();
                                let str_ref = self.memory.alloc_string(&char_str);
                                
                                self.memory.stack_push(str_ref);
                                ip += 2;    // Skip JUMP_IF_NOT
                                continue;
                            }
                        }
                        
                        self.memory.stack_push(DinoRef::FALSE);
                    }
                },
                
                opcode::FOR_DROP => {
                    let var_idx = payload as u32;
                    // var_idx: target
                    // var_idx + 1: current index
                    // var_idx + 2: limit/count
                    // var_idx + 3: step
                    unsafe {
                        self.memory.set_variable_unchecked(var_idx, DinoRef::NONE);
                        self.memory.set_variable_unchecked(var_idx + 1, DinoRef::NONE);
                        self.memory.set_variable_unchecked(var_idx + 2, DinoRef::NONE);
                        self.memory.set_variable_unchecked(var_idx + 3, DinoRef::NONE);
                    }
                },
                
                opcode::RETURN => {                                        
                    if let Some(return_address) = self.memory.pop_call_frame() {
                        ip = return_address as usize;
                        continue;
                    } else {
                        return Ok(());
                    }
                },
                
                opcode::RETURN_REF => {
                    if let Some(return_address) = self.memory.pop_call_frame_with_ref() {
                        ip = return_address as usize;
                        continue;
                    } else {
                        return Ok(());
                    }
                },
                
                opcode::RETURN_SELF => {
                    if let Some(return_address) = self.memory.pop_call_frame_with_self() {
                        ip = return_address as usize;
                        continue;
                    } else {
                        return Ok(());
                    }
                },
                
                opcode::MAKE_OBJECT => {
                    let count = payload as usize;
                    if count > self.memory.stack_depth() {
                        return vm_err!(RuntimeError::StackUnderflow);
                    }

                    let start = self.memory.stack_depth() - count;
                    let object_ref = Object::create_instance(&mut self.memory, start, count);

                    self.memory.stack_pop_n(count);
                    self.memory.stack_push(object_ref);
                },

                opcode::MAKE_CLASS => {
                    let method_count = payload as usize;
                    let stack_depth = self.memory.stack_depth();

                    if (method_count * 2) + 1 > stack_depth {
                        return vm_err!(RuntimeError::StackUnderflow);
                    }

                    let parent_pos = stack_depth - (method_count * 2) - 1;
                    let parent = try_vm!(self.memory.stack_get(parent_pos).ok_or(RuntimeError::StackUnderflow));

                    let start = parent_pos + 1;
                    let prop_count = method_count * 2;
                    let object_ref = Object::create_instance(&mut self.memory, start, prop_count);
                    let offset = object_ref.get_object_id();

                    self.memory.set_proto(offset, parent);
                    let class_ref = DinoRef::class(offset);

                    self.memory.stack_pop_n(prop_count + 1);
                    self.memory.stack_push(class_ref);
                },
                
                opcode::MAKE_RANGE => {
                    let inclusive = payload == 1;
                    
                    let end = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let start = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let start_int = try_vm!(start.try_as_int(&mut self.memory));
                    let mut end_int = try_vm!(end.try_as_int(&mut self.memory));
                    if inclusive {
                        end_int = end_int.saturating_add(1);
                    }
                    
                    let range_ref = Range::create_instance(&mut self.memory, start_int, end_int, 1);
                    
                    self.memory.stack_pop_n(2);
                    self.memory.stack_push(range_ref);
                },

                opcode::GET_MEMBER => {
                    let property_name = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.get_property(object, property_name) {
                        Ok(result) => {
                            runtime.memory.stack_pop_n(2);
                            runtime.memory.stack_push(result);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },

                opcode::GET_MEMBER_PREP => {
                    let property_name = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.get_property(object, property_name) {
                        Ok(result) => {
                            runtime.memory.stack_push(result);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },
                
                opcode::GET_INDEX => {
                    let property_name = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.get_index(object, property_name) {
                        Ok(result) => {
                            runtime.memory.stack_pop_n(2);
                            runtime.memory.stack_push(result);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },
                
                opcode::GET_METHOD => {
                    let property_name = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.get_property(object, property_name) {
                        Ok(method) => {
                            runtime.memory.stack_pop_n(2);
                            runtime.memory.stack_push(method);
                            runtime.memory.stack_push(object);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },
                
                opcode::GET_NATIVE_MEMBER => {
                    let property_name = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.get_native_property(object, property_name) {
                        Ok(result) => {
                            runtime.memory.stack_pop_n(2);
                            runtime.memory.stack_push(result);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },
                
                opcode::GET_NATIVE_METHOD => {
                    let property_name = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.get_native_property(object, property_name) {
                        Ok(method) => {
                            runtime.memory.stack_pop_n(2);
                            runtime.memory.stack_push(method);
                            runtime.memory.stack_push(object);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },
                
                opcode::SET_MEMBER => {
                    let value = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let property_name = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(2).ok_or(RuntimeError::StackUnderflow));
                    
                    let result = {
                        let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                        try_vm!(runtime.set_property(object, property_name, value))
                    };
                    
                    self.memory.stack_pop_n(3);
                    self.memory.stack_push(result);
                },

                opcode::SET_MEMBER_PREP => {
                    let value = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let property_name = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(2).ok_or(RuntimeError::StackUnderflow));
                    
                    let result = {
                        let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                        try_vm!(runtime.set_property(object, property_name, value))
                    };
                    
                    self.memory.stack_pop_n(3);
                    self.memory.stack_push(result);
                },
                
                opcode::SET_INDEX => {
                    let value = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let property_name = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(2).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.set_index(object, property_name, value) {
                        Ok(result) => {
                            runtime.memory.stack_pop_n(3);
                            runtime.memory.stack_push(result);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },

                opcode::GET_INDEX_PREP => {
                    let index = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.get_index(object, index) {
                        Ok(result) => {
                            runtime.memory.stack_push(result);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },

                opcode::SET_INDEX_PREP => {
                    let value = try_vm!(self.memory.stack_peek(0).ok_or(RuntimeError::StackUnderflow));
                    let index = try_vm!(self.memory.stack_peek(1).ok_or(RuntimeError::StackUnderflow));
                    let object = try_vm!(self.memory.stack_peek(2).ok_or(RuntimeError::StackUnderflow));
                    
                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    match runtime.set_index(object, index, value) {
                        Ok(result) => {
                            runtime.memory.stack_pop_n(3);
                            runtime.memory.stack_push(result);
                        },
                        Err(msg) => {
                            return vm_err!(msg);
                        }
                    }
                },

                opcode::INPUT => {
                    let stack_depth = self.memory.stack_depth();
                    let args_start = stack_depth - 1;
                    
                    let result = try_vm!(io::input(&mut self.memory, args_start, 1));
                    self.memory.stack_pop_n(2); // Expected as binary operation
                    self.memory.stack_push(result);
                },

                opcode::CALL => {
                    let argc = payload as usize;
                    let stack_len = self.memory.stack_depth();
                    
                    if argc + 1 > stack_len {
                        return vm_err!(RuntimeError::StackUnderflow);
                    }
                    
                    let args_start = stack_len - argc;
                    let function_pos = args_start - 1;
                    
                    let function_ref = try_vm!(self.memory.stack_get(function_pos)
                        .ok_or(RuntimeError::StackUnderflow));

                    let mut runtime = Runtime::new(&mut self.memory, &self.functions, &mut ip);
                    try_vm!(runtime.call(function_ref, args_start, argc, function_pos));
                },
                
                opcode::TO => {
                    let value = try_vm!(self.memory.stack_peek_top().ok_or(RuntimeError::StackUnderflow));
                    let target_type = payload;
                    
                    let converted = match target_type {
                        0 => try_vm!(TypeConverter::to_number(value, &mut self.memory)),    // number
                        1 => try_vm!(TypeConverter::to_int(value, &mut self.memory)),      // int
                        2 => try_vm!(TypeConverter::to_float(value, &mut self.memory)),    // float
                        3 => try_vm!(TypeConverter::to_string(value, &mut self.memory)),   // string
                        4 => try_vm!(TypeConverter::to_bool(value, &mut self.memory)),     // bool
                        5 => try_vm!(TypeConverter::to_bigint(value, &mut self.memory)),  // bigint
                        _ => return vm_err!(RuntimeError::TypeError(format!("Invalid type index: {}", target_type))),
                    };
                    
                    self.memory.stack_pop_n(1);
                    self.memory.stack_push(converted);
                },
                
                _ => return vm_err!(RuntimeError::InternalError(format!("Unimplemented opcode: 0x{:02X}", op))),
            }

            ip += 1;
        }
        
        Ok(())
    }
}
