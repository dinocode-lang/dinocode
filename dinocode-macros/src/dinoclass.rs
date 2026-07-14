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
            static mut [<_DINOCLASS_BOOTSTRAP_IDX_ #struct_name>]: Option<u32> = None;

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
                let handle = runtime.memory.alloc_object_capacity(8);
                
                paste::paste! {
                    if let Some(methods) = [<_DINOCLASS_METHODS_ #struct_name>].get() {
                        for &(method_name_raw, id, prop_flags) in methods {
                            let key = runtime.memory.alloc_const_string(method_name_raw);
                            let val = crate::types::DinoRef::native_fn(id);
                            let _ = runtime.memory.set_object_property(handle, key, val, prop_flags);
                        }
                    }
                    if let Some(symbols) = [<_DINOCLASS_SYMBOLS_ #struct_name>].get() {
                        for &(sym_key, id, prop_flags) in symbols {
                            let val = crate::types::DinoRef::native_fn(id);
                            let _ = runtime.memory.set_object_property(handle, sym_key, val, prop_flags);
                        }
                    }
                    if let Some(keys) = [<_DINOCLASS_KEYS_ #struct_name>].get() {
                        for &(key_name_raw, setter) in keys {
                            let key = runtime.memory.alloc_const_string(key_name_raw);
                            setter(key);
                        }
                    }
                    if let Some(props) = [<_DINOCLASS_PROPS_ #struct_name>].get() {
                        for &(prop_name_raw, val, prop_flags) in props {
                            let key = runtime.memory.alloc_const_string(prop_name_raw);
                            let _ = runtime.memory.set_object_property(handle, key, val, prop_flags);
                        }
                    }
                }
                
                #[cfg(feature = "logging")]
                log::debug!(" DinoClass: Bootstrap complete for {}", stringify!(#struct_name));
                
                let class_ref = if #is_static {
                    crate::types::DinoRef::object(handle)
                } else {
                    crate::types::DinoRef::class(handle)
                };
                Ok(class_ref)
            }
            
            pub fn get_bootstrap_index() -> Option<u32> {
                unsafe { 
                    paste::paste! { [<_DINOCLASS_BOOTSTRAP_IDX_ #struct_name>] }
                }
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
                    [<_DINOCLASS_BOOTSTRAP_IDX_ #struct_name>] = Some(idx);
                }
            }
        }
    };
    
    TokenStream::from(generated)
}
