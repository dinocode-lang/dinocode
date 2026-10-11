// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/dinoclass.rs
//  Desc:       dinoclass struct macro implementation
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn dinoclass(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input_struct.ident;
    
    let attr_string = attr.to_string();
    let is_static = attr_string.contains("static");
    
    let generated = quote! {
        #input_struct
        
        paste::paste! {
            #[allow(non_upper_case_globals)]
            static mut [<_DINOCLASS_BOOTSTRAP_IDX_ #struct_name>]: u32 = u32::MAX;
            #[allow(non_upper_case_globals)]
            static [<_DINOCLASS_METHODS_ #struct_name>]: std::sync::OnceLock<
                Vec<(&'static str, u32, u8)>
            > = std::sync::OnceLock::new();

            #[allow(non_upper_case_globals)]
            static [<_DINOCLASS_SYMBOLS_ #struct_name>]: std::sync::OnceLock<
                Vec<(crate::types::DinoRef, u32, u8)>
            > = std::sync::OnceLock::new();

            #[allow(non_upper_case_globals)]
            static [<_DINOCLASS_PROPS_ #struct_name>]: std::sync::OnceLock<
                Vec<(&'static str, crate::types::DinoRef, u8)>
            > = std::sync::OnceLock::new();

            #[allow(non_upper_case_globals)]
            static [<_DINOCLASS_KEYS_ #struct_name>]: std::sync::OnceLock<
                Vec<(&'static str, fn(crate::types::DinoRef))>
            > = std::sync::OnceLock::new();
        }
        
        impl #struct_name {
            pub fn create_class_prototype(
                runtime: &mut crate::runtime::context::Runtime,
                _args_start: usize,
                _args_count: usize
            ) -> crate::errors::Result<crate::types::DinoRef> {
                use crate::builtins::object::{Object, view::ObjectView};
                use crate::memory::types::property_flags::PROP_PROTOTYPE;
                let proto = Object::as_ref(&mut runtime.memory);
                let class = ObjectView::new_from_slice(proto, &[], PROP_PROTOTYPE, &mut runtime.memory)?;
                paste::paste! {
                    if let Some(methods) = [<_DINOCLASS_METHODS_ #struct_name>].get() {
                        for &(method_name_raw, id, prop_flags) in methods {
                            let key = crate::builtins::string::view::StringView::new_const(method_name_raw, &mut runtime.memory).as_ref();
                            let val = crate::types::DinoRef::native_fn(id);
                            class.set(key, val, prop_flags, &mut runtime.memory)?;
                        }
                    }
                    if let Some(symbols) = [<_DINOCLASS_SYMBOLS_ #struct_name>].get() {
                        for &(sym_key, id, prop_flags) in symbols {
                            let val = crate::types::DinoRef::native_fn(id);
                            class.set(sym_key, val, prop_flags, &mut runtime.memory)?;
                        }
                    }
                    if let Some(keys) = [<_DINOCLASS_KEYS_ #struct_name>].get() {
                        for &(key_name_raw, setter) in keys {
                            let key = crate::builtins::string::view::StringView::new_const(key_name_raw, &mut runtime.memory).as_ref();
                            setter(key);
                        }
                    }
                    if let Some(props) = [<_DINOCLASS_PROPS_ #struct_name>].get() {
                        for &(prop_name_raw, val, prop_flags) in props {
                            let key = crate::builtins::string::view::StringView::new_const(prop_name_raw, &mut runtime.memory).as_ref();
                            class.set(key, val, prop_flags, &mut runtime.memory)?;
                        }
                    }
                }
                
                #[cfg(feature = "logging")]
                log::debug!(" DinoClass: Bootstrap complete for {}", stringify!(#struct_name));
                
                let class_ref = if #is_static {
                    class.as_ref()
                } else {
                    class.as_class_ref()
                };
                Ok(class_ref)
            }
            
            pub fn get_bootstrap_index() -> u32 {
                let idx = unsafe {
                    paste::paste! { [<_DINOCLASS_BOOTSTRAP_IDX_ #struct_name>] }
                };
                #[cfg(feature = "logging")]
                if idx == u32::MAX {
                    panic!("Bootstrap not initialized for {}", stringify!(#struct_name));
                }
                idx
            }
            pub fn as_ref(memory: &mut crate::memory::MemoryManager) -> crate::types::DinoRef {
                let idx = Self::get_bootstrap_index();
                unsafe { memory.stack.get_unchecked(idx as usize) }
            }
        }

        paste::paste! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            fn [<_dinoclass_init_ #struct_name>]() {
                let class_name = stringify!(#struct_name).to_lowercase();
                
                let idx = crate::native::register_native_class(
                    &class_name,
                    #struct_name::create_class_prototype
                );
                
                unsafe {
                    [<_DINOCLASS_BOOTSTRAP_IDX_ #struct_name>] = idx;
                }
            }
        }
    };
    
    TokenStream::from(generated)
}
