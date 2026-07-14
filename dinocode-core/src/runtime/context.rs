// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/runtime/context.rs
//  Desc:       Runtime context for execution
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        MemoryManager,
        types::PropertyFlags,
    },
    types::{DinoRef, UserFunction, Symbol, value_type},
    native::call_native_function,
    errors::{Result, RuntimeError},
    prototypes::{
        string::String as StringProto,
        array::Array as ProtoArray,
        object::Object,
    },
};

pub struct Runtime<'a, 'b> {
    pub memory: &'a mut MemoryManager,
    pub functions: &'b Vec<UserFunction>,
    pub ip: &'a mut usize,
}

impl<'a, 'b> Runtime<'a, 'b> {
    pub fn new(
        memory: &'a mut MemoryManager,
        functions: &'b Vec<UserFunction>,
        ip: &'a mut usize,
    ) -> Self {
        Self { memory, functions, ip }
    }

    pub fn call(&mut self, function_ref: DinoRef, args_start: usize, argc: usize, function_pos: usize) -> Result<()> {
        match function_ref.decode_type() {
            value_type::FUNCTION => {
                if function_ref.is_native_fn() {
                    let fid = function_ref.get_function_id();
                    let res = call_native_function(self, fid, args_start, argc)?;
                    
                    self.memory.move_sp(function_pos);
                    self.memory.stack_push(res);
                } else {
                    let function_id = function_ref.get_function_id();
                    
                    if let Some(function) = self.functions.get(function_id as usize) {
                        let start_ip = function.start_ip;
                        
                        self.memory.push_call_frame(
                            (*self.ip + 1) as u32,
                            function_id,
                            function,
                            args_start,
                            argc
                        )?;
                        
                        *self.ip = start_ip as usize - 1;
                    } else {
                        return Err(RuntimeError::FunctionNotFound);
                    }
                }
            }
            value_type::OBJECT => {
                if function_ref.is_class() {
                    let class_id = function_ref.get_object_id();
                    
                    let instance = Object::create_from_slice(self, &[]);
                    self.memory.set_proto(instance.get_object_id(), function_ref);
                    
                    if let Some(new_method) = self.memory.get_property(class_id, Symbol::NEW) {
                        unsafe { self.memory.stack_set_unchecked(function_pos, instance); }
                        self.memory.stack_insert(function_pos, new_method);
                        
                        let new_argc = argc + 1;
                        let new_args_start = function_pos + 1;
                        
                        if new_method.is_user_function() {
                            let fid = new_method.get_function_id();
                            if let Some(func) = self.functions.get(fid as usize) {
                                self.memory.push_call_frame((*self.ip + 1) as u32, fid, func, new_args_start, new_argc)?;
                                *self.ip = func.start_ip as usize - 1;
                                return Ok(());
                            }
                        } else if new_method.is_native_fn() {
                            let fid = new_method.get_function_id();
                            let res = call_native_function(self, fid, new_args_start, new_argc)?;
                            self.memory.move_sp(function_pos);
                            self.memory.stack_push(res);
                            self.memory.stack_push(instance);
                        }

                        return Ok(());
                    } 
                    self.memory.move_sp(function_pos);
                    self.memory.stack_push(instance);
                } else {
                    let object_id = function_ref.get_object_id();
                    
                    if let Some(call_method) = self.memory.get_property(object_id, Symbol::CALL) {
                        unsafe { self.memory.stack_set_unchecked(function_pos, function_ref); }
                        self.memory.stack_insert(function_pos, call_method);
                        
                        let call_argc = argc + 1;
                        let call_args_start = function_pos + 1;
                        
                        if call_method.is_user_function() {
                            let fid = call_method.get_function_id();
                            if let Some(func) = self.functions.get(fid as usize) {
                                self.memory.push_call_frame((*self.ip + 1) as u32, fid, func, call_args_start, call_argc)?;
                                *self.ip = func.start_ip as usize - 1;
                                return Ok(());
                            }
                        } else if call_method.is_native_fn() {
                            let fid = call_method.get_function_id();
                            let res = call_native_function(self, fid, call_args_start, call_argc)?;
                            self.memory.move_sp(function_pos);
                            self.memory.stack_push(res);
                            return Ok(());
                        }
                    } else {
                        return Err(RuntimeError::CallNotFunction(function_ref.type_name()));
                    }
                }
            }
            _ => {
                return Err(RuntimeError::CallNotFunction(function_ref.type_name()));
            }
        }
        
        Ok(())
    }

    pub fn call_function(&mut self, function_ref: DinoRef, args_start: usize, argc: usize) -> Result<DinoRef> {
        if function_ref.is_native_fn() {
            let native_id = function_ref.get_function_id();
            call_native_function(self, native_id, args_start, argc)
        } else if function_ref.is_user_function() {
            let function_id = function_ref.get_function_id();
            if let Some(function) = self.functions.get(function_id as usize) {
                let start_ip = function.start_ip;
                if let Err(e) = self.memory.push_call_frame(
                    *self.ip as u32,
                    function_id,
                    function,
                    args_start,
                    argc
                ) {
                    return Err(e);
                }
                *self.ip = start_ip as usize;
                Ok(DinoRef::NONE)
            } else {
                Err(RuntimeError::FunctionNotFound)
            }
        } else {
            Err(RuntimeError::InvalidArgumentValue {
                func: "call_function",
                message: "expected a function reference",
            })
        }
    }

    fn try_execute_getter(&mut self, getter: DinoRef) -> Result<DinoRef> {
        let stack_depth = self.memory.stack_depth();
        let args_start = stack_depth - 2;
        self.call_function(getter, args_start, 1)
    }

    fn try_execute_setter(&mut self, setter: DinoRef, value: DinoRef) -> Result<DinoRef> {
        let stack_depth = self.memory.stack_depth();
        self.memory.stack_set(stack_depth - 2, value);
        self.call_function(setter, stack_depth - 3, 2)
    }

    fn try_set_property_via_setter(&mut self, handle: u32, property_name: DinoRef, value: DinoRef) -> Result<Option<DinoRef>> {
        if let Some((setter, flags)) = self.memory.get_property_details(handle, property_name) {
            let flags = PropertyFlags::from(flags);
            if flags.is_setter() {
                return self.try_execute_setter(setter, value).map(Some);
            }
        }
        Ok(None)
    }

    pub fn get_property_by_id(&mut self, object_id: u32, property_name: DinoRef) -> Result<DinoRef> {
        if let Some((value, flags)) = self.memory.get_property_details(object_id, property_name) {
            let flags = PropertyFlags::from(flags);
            if flags.is_getter() {
                self.try_execute_getter(value)
            } else {
                Ok(value)
            }
        } else {
            let name_str = property_name.to_key_string(self.memory)?;
            Err(RuntimeError::PropertyNotFound(name_str))
        }
    }

    pub fn get_property(&mut self, object: DinoRef, property_name: DinoRef) -> Result<DinoRef> {
        match object.decode_type() {
            value_type::ARRAY => {
                let proto = self.memory.get_proto(object.decode_index());
                if proto.is_object() {
                    self.get_property_by_id(proto.get_object_id(), property_name)
                } else {
                    let name_str = property_name.to_key_string(self.memory)?;
                    Err(RuntimeError::PropertyNotFound(name_str))
                }
            }
            value_type::OBJECT => {
                self.get_property_by_id(object.get_object_id(), property_name)
            }
            value_type::STRING => {
                if let Some(stack_idx) = StringProto::get_bootstrap_index() {
                    let proto_ref = unsafe { self.memory.get_global_variable_unchecked(stack_idx) };
                    let proto_id = proto_ref.get_object_id();
                    self.get_property_by_id(proto_id, property_name)
                } else {
                    Err(RuntimeError::InternalError("String prototype bootstrap not available"))
                }
            }
            _ => Err(RuntimeError::MemberAccessNotObject)
        }
    }

    pub fn set_property(&mut self, object: DinoRef, property_name: DinoRef, value: DinoRef) -> Result<DinoRef> {
        match object.decode_type() {
            value_type::ARRAY => {
                let proto = self.memory.get_proto(object.decode_index());
                if proto.is_object() {
                    let proto_id = proto.get_object_id();
                    if let Some(res) = self.try_set_property_via_setter(proto_id, property_name, value)? {
                        Ok(res)
                    } else {
                        let name_str = property_name.to_key_string(self.memory)?;
                        Err(RuntimeError::PropertyNotFound(name_str))
                    }
                } else {
                    let name_str = property_name.to_key_string(self.memory)?;
                    Err(RuntimeError::PropertyNotFound(name_str))
                }
            }
            value_type::OBJECT => {
                let object_id = object.get_object_id();
                if let Some(res) = self.try_set_property_via_setter(object_id, property_name, value)? {
                    Ok(res)
                } else {
                    self.memory.set_object_property(object_id, property_name, value, 0)?;
                    Ok(value)
                }
            }
            _ => Err(RuntimeError::MemberAccessNotObject)
        }
    }

    pub fn get_index(&mut self, object: DinoRef, index_ref: DinoRef) -> Result<DinoRef> {
        match object.decode_type() {
            value_type::ARRAY => {
                let index = index_ref.try_as_int(self.memory)?;
                if index < 0 {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                
                let array_offset = object.decode_index();
                let array_len = self.memory.get_array_len(array_offset);
                
                if index as u32 >= array_len {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                
                let element = self.memory.get_array_element(array_offset, index as u32);
                Ok(element)
            }
            value_type::OBJECT => {
                self.get_property_by_id(object.get_object_id(), index_ref)
            }
            value_type::STRING => {
                if let Ok(index) = index_ref.try_as_int(self.memory) {
                    if index < 0 {
                        return Err(RuntimeError::IndexOutOfBounds);
                    }
                    let s = self.memory.get_string(object.decode_index());
                    if let Some(c) = s.chars().nth(index as usize) {
                        return Ok(self.memory.alloc_string(&c.to_string()));
                    } else {
                        return Err(RuntimeError::IndexOutOfBounds);
                    }
                }
                
                if let Some(stack_idx) = StringProto::get_bootstrap_index() {
                    let proto_ref = unsafe { self.memory.get_global_variable_unchecked(stack_idx) };
                    let proto_id = proto_ref.get_object_id();
                    self.get_property_by_id(proto_id, index_ref)
                } else {
                    Err(RuntimeError::InternalError("String prototype bootstrap not available"))
                }
            }
            _ => Err(RuntimeError::MemberAccessNotObject)
        }
    }

    pub fn set_index(&mut self, object: DinoRef, index_ref: DinoRef, value: DinoRef) -> Result<DinoRef> {
        match object.decode_type() {
            value_type::ARRAY => {
                let index = index_ref.try_as_int(self.memory)?;
                if index < 0 {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                
                let array_offset = object.decode_index();
                self.memory.set_array_element(array_offset, index as u32, value)?;
                Ok(value)
            }
            value_type::OBJECT => {
                let object_id = object.get_object_id();
                if let Some(res) = self.try_set_property_via_setter(object_id, index_ref, value)? {
                    Ok(res)
                } else {
                    self.memory.set_object_property(object_id, index_ref, value, 0)?;
                    Ok(value)
                }
            }
            _ => Err(RuntimeError::MemberAccessNotObject)
        }
    }

    pub fn get_native_property(&mut self, object: DinoRef, property_name: DinoRef) -> Result<DinoRef> {
        match object.decode_type() {
            value_type::STRING => {
                if let Some(stack_idx) = StringProto::get_bootstrap_index() {
                    let proto = unsafe { self.memory.get_global_variable_unchecked(stack_idx) };
                    
                    if proto.is_object() {
                        let proto_id = proto.get_object_id();
                        
                        if let Some((value, flags)) = self.memory.get_property_details(proto_id, property_name) {
                            let flags = PropertyFlags::from(flags);
                            
                            if flags.is_getter() {
                                let args_start = self.memory.stack_depth() - 2;
                                self.call_function(value, args_start, 1)
                            } else {
                                Ok(value)
                            }
                        } else {
                            let name_str = property_name.to_key_string(self.memory)?;
                            Err(RuntimeError::PropertyNotFound(name_str))
                        }
                    } else {
                        Err(RuntimeError::InternalError("String prototype not found"))
                    }
                } else {
                    Err(RuntimeError::InternalError("String prototype not initialized"))
                }
            },
            value_type::ARRAY => {
                if let Some(stack_idx) = ProtoArray::get_bootstrap_index() {
                    let proto = unsafe { self.memory.get_global_variable_unchecked(stack_idx) };
                    
                    if proto.is_object() {
                        let proto_id = proto.get_object_id();
                        
                        if let Some((value, flags)) = self.memory.get_property_details(proto_id, property_name) {
                            let flags = PropertyFlags::from(flags);
                            
                            if flags.is_getter() {
                                let args_start = self.memory.stack_depth() - 2;
                                self.call_function(value, args_start, 1)
                            } else {
                                Ok(value)
                            }
                        } else {
                            let name_str = property_name.to_key_string(self.memory)?;
                            Err(RuntimeError::PropertyNotFound(name_str))
                        }
                    } else {
                        Err(RuntimeError::InternalError("Array prototype not found"))
                    }
                } else {
                    Err(RuntimeError::InternalError("Array prototype not initialized"))
                }
            },
            value_type::OBJECT => {
                let object_offset = object.get_object_id();
                let proto = self.memory.get_proto(object_offset);
                
                if proto.is_object() {
                    let proto_id = proto.get_object_id();
                    
                    if let Some((value, flags)) = self.memory.get_property_details(proto_id, property_name) {
                        let flags = PropertyFlags::from(flags);
                        
                        if flags.is_getter() {
                            let args_start = self.memory.stack_depth() - 2;
                            self.call_function(value, args_start, 1)
                        } else {
                            Ok(value)
                        }
                    } else {
                        let name_str = property_name.to_key_string(self.memory)?;
                        Err(RuntimeError::PropertyNotFound(name_str))
                    }
                } else {
                    Err(RuntimeError::InternalError("Object does not have native prototype"))
                }
            },
            _ => Err(RuntimeError::ExpectedInstance("object"))
        }
    }
}
