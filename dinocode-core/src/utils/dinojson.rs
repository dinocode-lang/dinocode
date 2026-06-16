// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/dinojson.rs
//  Desc:       Utility for styling arrays and objects
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    errors::RuntimeError,
    types::{DinoRef, value_type},
    utils::TypeConverter,
};

pub struct DinoJsonFormatter<'a> {
    memory: &'a MemoryManager,
    indent_size: usize,
    current_indent: usize,
}

impl<'a> DinoJsonFormatter<'a> {
    pub fn new(memory: &'a MemoryManager) -> Self {
        Self {
            memory,
            indent_size: 2,
            current_indent: 0,
        }
    }

    pub fn with_indent(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    pub fn format(&self, value: DinoRef) -> Result<String, RuntimeError> {
        let mut result = String::with_capacity(256);
        self.format_value(value, self.current_indent, &mut result)?;
        Ok(result)
    }

    fn format_value(&self, value: DinoRef, indent: usize, result: &mut String) -> Result<(), RuntimeError> {
        match value.decode_type() {
            value_type::ARRAY => self.format_array_into(value, indent, result),
            value_type::OBJECT => self.format_object_into(value, indent, result),
            _ => {
                let formatted = value.to_display_string(self.memory)?;
                result.push_str(&formatted);
                Ok(())
            }
        }
    }

    fn format_array_into(&self, array_ref: DinoRef, indent: usize, result: &mut String) -> Result<(), RuntimeError> {
        let temp_indent = indent + 1;
        let elements = self.get_array_elements(array_ref);
        
        if elements.is_empty() {
            result.push_str("[]");
            return Ok(());
        }

        result.push_str("[\n");
        result.push_str(&self.indent_string(temp_indent));

        for (i, &element) in elements.iter().enumerate() {
            self.format_value(element, temp_indent, result)?;
            
            if i < elements.len() - 1 {
                result.push_str(",\n");
                result.push_str(&self.indent_string(temp_indent));
            }
        }

        result.push('\n');
        result.push_str(&self.indent_string(indent));
        result.push(']');

        Ok(())
    }

    fn format_object_into(&self, object_ref: DinoRef, indent: usize, result: &mut String) -> Result<(), RuntimeError> {
        let temp_indent = indent + 1;
        let properties = self.get_object_properties(object_ref)?;
        
        if properties.is_empty() {
            result.push_str("{}");
            return Ok(());
        }

        result.push_str("{\n");
        result.push_str(&self.indent_string(temp_indent));

        for (i, (key, value, key_ref)) in properties.iter().enumerate() {
            let key_vtype = key_ref.decode_type();
            if key_vtype == value_type::STRING {
                result.push('"');
                result.push_str(key);
                result.push_str("\": ");
            } else {
                result.push_str(key);
                result.push_str(": ");
            }

            self.format_value(*value, temp_indent, result)?;
            
            if i < properties.len() - 1 {
                result.push_str(",\n");
                result.push_str(&self.indent_string(temp_indent));
            }
        }

        result.push('\n');
        result.push_str(&self.indent_string(indent));
        result.push('}');

        Ok(())
    }

    fn indent_string(&self, level: usize) -> String {
        " ".repeat(level * self.indent_size)
    }

    fn get_array_elements(&self, array_ref: DinoRef) -> Vec<DinoRef> {
        let array_id = array_ref.decode_index();
        let array_len = self.memory.get_array_len(array_id);
        let mut elements = Vec::with_capacity(array_len as usize);
        
        for i in 0..array_len {
            let element = self.memory.get_array_element(array_id, i);
            elements.push(element);
        }
        
        elements
    }

    fn get_object_properties(&self, object_ref: DinoRef) -> Result<Vec<(String, DinoRef, DinoRef)>, RuntimeError> {
        let object_id = object_ref.get_object_id();
        let mut properties = Vec::new();

        let keys = self.memory.get_object_keys(object_id);
        let values = self.memory.get_object_values(object_id);
        for (key_ref, value) in keys.iter().zip(values.iter()) {
            let key_str = TypeConverter::to_key_string(*key_ref, self.memory)?;
            properties.push((key_str, *value, *key_ref));
        }

        properties.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(properties)
    }
}

impl<'a> Clone for DinoJsonFormatter<'a> {
    fn clone(&self) -> Self {
        Self {
            memory: self.memory,
            indent_size: self.indent_size,
            current_indent: self.current_indent,
        }
    }
}

pub fn dinojson(value: DinoRef, memory: &MemoryManager) -> Result<String, RuntimeError> {
    let formatter = DinoJsonFormatter::new(memory);
    formatter.format(value)
}
