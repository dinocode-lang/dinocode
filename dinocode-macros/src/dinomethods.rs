// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/dinomethods.rs
//  Desc:       dinomethods and dinomethod macro implementations
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, ImplItem, FnArg, Pat};

pub fn dinomethods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_impl = parse_macro_input!(item as ItemImpl);
    
    let self_ty = &input_impl.self_ty;
    let self_ty_str = quote!(#self_ty).to_string();
    let struct_name = self_ty_str.replace(" ", "");

    let mut generated_methods = Vec::new();
    let mut generated_props = Vec::new();
    let mut generated_keys = Vec::new();
    let mut kept_items = Vec::new();
    let mut static_storages = Vec::new();
    
    for impl_item in &input_impl.items {
        if let ImplItem::Const(const_item) = impl_item {
            let mut is_prop = false;
            let mut is_key = false;
            let mut prop_with_key = false;

            for attr in &const_item.attrs {
                if attr.path().is_ident("prop") {
                    is_prop = true;
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("key") {
                            prop_with_key = true;
                        }
                        Ok(())
                    });
                } else if attr.path().is_ident("key") {
                    is_key = true;
                }
            }

            if is_prop || is_key {
                let const_ident = &const_item.ident;
                let prop_name = const_ident.to_string().to_lowercase();
                let prop_val = &const_item.expr;
                
                if is_key || prop_with_key {
                    let storage_ident = quote::format_ident!("_DINOCLASS_KEY_{}_{}", struct_name, const_ident);
                    
                    generated_keys.push(quote! {
                        _keys.push((
                            #prop_name, 
                            |k: crate::types::DinoRef| { let _ = #storage_ident.set(k); }
                        ));
                    });

                    static_storages.push(quote! {
                        #[allow(non_upper_case_globals)]
                        #[doc(hidden)]
                        pub static #storage_ident: std::sync::OnceLock<crate::types::DinoRef> = std::sync::OnceLock::new();
                    });

                    let vis = &const_item.vis;
                    kept_items.push(syn::parse_quote! {
                        #[inline(always)]
                        #[allow(non_snake_case)]
                        #vis fn #const_ident() -> crate::types::DinoRef {
                            *#storage_ident.get().unwrap()
                        }
                    });
                }
                
                if is_prop {
                    generated_props.push(quote! {
                        _props.push((#prop_name, #prop_val, 0u8));
                    });
                }
            } else {
                kept_items.push(impl_item.clone());
            }
            continue;
        }

        kept_items.push(impl_item.clone());

        if let ImplItem::Fn(method) = impl_item {
            let method_name = &method.sig.ident;
            
            let mut is_symbol = false;
            let mut custom_symbol_name = None;
            let mut keep_method_name = false;
            for attr in &method.attrs {
                if attr.path().is_ident("symbol") {
                    is_symbol = true;
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("name") {
                            let value = meta.value()?;
                            let s: syn::LitStr = value.parse()?;
                            custom_symbol_name = Some(s.value());
                        } else if meta.path.is_ident("alias") {
                            keep_method_name = true;
                        }
                        Ok(())
                    });
                }
            }
            
            let sym_name_str = custom_symbol_name.clone().unwrap_or_else(|| method_name.to_string());
            let registered_name = if is_symbol {
                format!("{}_{}{}", struct_name, "__symbol__", sym_name_str)
            } else {
                format!("{}_{}", struct_name, method_name)
            };
            
            let method_key: String = method_name.to_string().to_lowercase();
            let method_key_alias: String = method_name.to_string().to_lowercase();
            
            let is_raw = method.attrs.iter().any(|attr| attr.path().is_ident("raw"));
            let is_getter = method.attrs.iter().any(|attr| attr.path().is_ident("getter"));
            let is_setter = method.attrs.iter().any(|attr| attr.path().is_ident("setter"));
            
            let mut flags = 1u8;
            if is_getter { flags |= 0x04; }
            if is_setter { flags |= 0x08; }

            let prop_flags: u8 = if is_getter { 0x01 } else { 0 } | if is_setter { 0x02 } else { 0 };
            
            let mut push_logic = quote! {};
            if is_symbol {
                push_logic.extend(quote! {
                    let sym_val = crate::types::Symbol::from_name(#sym_name_str).unwrap_or_else(|| panic!("Invalid symbol name: {}", #sym_name_str));
                    _symbols.push((sym_val, _id, #prop_flags));
                });
            } else {
                push_logic.extend(quote! {
                    _methods.push((#method_key, _id, #prop_flags));
                });
            }

            let mut push_alias_logic = quote! {};
            if keep_method_name {
                let method_registered_name = format!("{}_{}", struct_name, method_name);
                push_alias_logic.extend(quote! {
                    {
                        let _id = crate::native::register_native_function_with_flags(
                            #method_registered_name,
                            wrapper_alias,
                            #flags
                        );
                        _methods.push((#method_key_alias, _id, #prop_flags));
                    }
                });
            }
            
            if is_raw {
                if keep_method_name {
                    let method_registered_name = format!("{}_{}", struct_name, method_name);
                    generated_methods.push(quote! {
                        {
                            let _id = crate::native::register_native_function_with_flags(
                                #registered_name,
                                #self_ty::#method_name,
                                #flags
                            );
                            #push_logic
                        }
                        {
                            let _id = crate::native::register_native_function_with_flags(
                                #method_registered_name,
                                #self_ty::#method_name,
                                #flags
                            );
                            _methods.push((#method_key_alias, _id, #prop_flags));
                        }
                    });
                } else {
                    generated_methods.push(quote! {
                        {
                            let _id = crate::native::register_native_function_with_flags(
                                #registered_name,
                                #self_ty::#method_name,
                                #flags
                            );
                            #push_logic
                        }
                    });
                }
            } else {
                let mut param_names = Vec::new();
                let mut arg_processing = Vec::new();
                
                for (idx, arg) in method.sig.inputs.iter().enumerate() {
                    match arg {
                        FnArg::Receiver(receiver) => {
                            let is_ref = receiver.reference.is_some();
                            
                            if is_ref {
                                if receiver.mutability.is_some() {
                                    param_names.push(quote!(&mut self_arg));
                                } else {
                                    param_names.push(quote!(&self_arg));
                                }
                            } else {
                                param_names.push(quote!(self_arg));
                            }
                            
                            arg_processing.push(quote! {
                                let mut self_arg = {
                                    let arg_val = memory.stack()[args_start + #idx];
                                    <#self_ty as crate::native::FromDinoRef>::from_dinoref(arg_val, memory)?
                                };
                            });
                        },
                        FnArg::Typed(pat_type) => {
                            let ty = &pat_type.ty;
                            let pat = &pat_type.pat;
                            if let Pat::Ident(ident) = &**pat {
                                let name = &ident.ident;
                                param_names.push(quote!(#name));
                                
                                arg_processing.push(quote! {
                                    let #name = {
                                        let arg_val = memory.stack()[args_start + #idx];
                                        <#ty as crate::native::FromDinoRef>::from_dinoref(arg_val, memory)?
                                    };
                                });
                            }
                        }
                    }
                }
                
                let call_stmt = quote! {
                    let result = #self_ty::#method_name( #(#param_names),* );
                };
                
                let param_count = param_names.len();
                let expected_count = if is_getter {
                    1usize
                } else if is_setter {
                    2usize
                } else if param_count == 0 {
                    1usize
                } else {
                    param_count
                };
                
                let wrapper_fn = quote! {
                    let wrapper = move |memory: &mut crate::memory::MemoryManager,
                                       args_start: usize,
                                       args_count: usize|
                          -> crate::errors::Result<crate::types::DinoRef> {
                        
                        if args_count != #expected_count {
                            return Err(crate::errors::RuntimeError::TypeError(
                                format!(
                                    "Expected {} arguments, got {}",
                                    #expected_count,
                                    args_count
                                )
                            ));
                        }
                        
                        #(#arg_processing)*
                        #call_stmt
                        ToDinoRef::to_dinoref(result, memory)
                    };
                };

                let alias_wrapper = if keep_method_name {
                    quote! {
                        let wrapper_alias = move |memory: &mut crate::memory::MemoryManager,
                                           args_start: usize,
                                           args_count: usize|
                              -> crate::errors::Result<crate::types::DinoRef> {
                            
                            if args_count != #expected_count {
                                return Err(crate::errors::RuntimeError::TypeError(
                                    format!(
                                        "Expected {} arguments, got {}",
                                        #expected_count,
                                        args_count
                                    )
                                ));
                            }
                            
                            #(#arg_processing)*
                            #call_stmt
                            ToDinoRef::to_dinoref(result, memory)
                        };
                    }
                } else { quote! {} };
                
                generated_methods.push(quote! {
                    {
                        use crate::native::{FromDinoRef, ToDinoRef};
                        #wrapper_fn
                        let _id = crate::native::register_native_function_with_flags(
                            #registered_name,
                            wrapper,
                            #flags
                        );
                        #push_logic

                        #alias_wrapper
                        #push_alias_logic
                    }
                });
            }
        }
    }
    
    input_impl.items = kept_items;
    
    let struct_name_ident: proc_macro2::TokenStream = struct_name.parse().unwrap();
    
    let expanded = quote! {
        #(#static_storages)*
        
        #input_impl
        
        paste::paste! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            pub fn [<_dinoclass_init_methods_ #struct_name_ident>]() {
                let mut _methods: Vec<(&'static str, u32, u8)> = Vec::new();
                let mut _symbols: Vec<(crate::types::DinoRef, u32, u8)> = Vec::new();
                let mut _props: Vec<(&'static str, crate::types::DinoRef, u8)> = Vec::new();
                let mut _keys: Vec<(&'static str, fn(crate::types::DinoRef))> = Vec::new();
                #(#generated_methods)*
                #(#generated_props)*
                #(#generated_keys)*
                let _ = [<_DINOCLASS_METHODS_ #struct_name_ident>].set(_methods);
                let _ = [<_DINOCLASS_SYMBOLS_ #struct_name_ident>].set(_symbols);
                let _ = [<_DINOCLASS_PROPS_ #struct_name_ident>].set(_props);
                let _ = [<_DINOCLASS_KEYS_ #struct_name_ident>].set(_keys);
            }
        }
    };
    
    TokenStream::from(expanded)
}

pub fn dinomethod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_impl = parse_macro_input!(item as ItemImpl);
    
    let self_ty = &input_impl.self_ty;
    let self_ty_str = quote!(#self_ty).to_string();
    let struct_name = self_ty_str.replace(" ", "");

    let mut generated_methods = Vec::new();
    
    for impl_item in &input_impl.items {
        if let ImplItem::Fn(method) = impl_item {
            let method_name = &method.sig.ident;
            
            let mut is_symbol = false;
            let mut custom_symbol_name = None;
            let mut keep_method_name = false;
            for attr in &method.attrs {
                if attr.path().is_ident("symbol") {
                    is_symbol = true;
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("name") {
                            let value = meta.value()?;
                            let s: syn::LitStr = value.parse()?;
                            custom_symbol_name = Some(s.value());
                        } else if meta.path.is_ident("alias") {
                            keep_method_name = true;
                        }
                        Ok(())
                    });
                }
            }
            
            let sym_name_str = custom_symbol_name.clone().unwrap_or_else(|| method_name.to_string());
            let registered_name = if is_symbol {
                format!("{}_{}{}", struct_name, "__symbol__", sym_name_str)
            } else {
                format!("{}_{}", struct_name, method_name)
            };

            let method_key: String = method_name.to_string().to_lowercase();
            let method_key_alias: String = method_name.to_string().to_lowercase();
            
            let mut param_names = Vec::new();
            let mut arg_processing = Vec::new();
            
            for (idx, arg) in method.sig.inputs.iter().enumerate() {
                match arg {
                    FnArg::Receiver(receiver) => {
                        let is_ref = receiver.reference.is_some();
                        
                        if is_ref {
                            if receiver.mutability.is_some() {
                                param_names.push(quote!(&mut self_arg));
                            } else {
                                param_names.push(quote!(&self_arg));
                            }
                        } else {
                            param_names.push(quote!(self_arg));
                        }
                        
                        arg_processing.push(quote! {
                            let mut self_arg = {
                                let arg_val = memory.stack()[args_start + #idx];
                                <#self_ty as crate::native::FromDinoRef>::from_dinoref(arg_val, memory)?
                            };
                        });
                    },
                    FnArg::Typed(pat_type) => {
                        let ty = &pat_type.ty;
                        let pat = &pat_type.pat;
                        if let Pat::Ident(ident) = &**pat {
                            let name = &ident.ident;
                            param_names.push(quote!(#name));
                            
                            arg_processing.push(quote! {
                                let #name = {
                                    let arg_val = memory.stack()[args_start + #idx];
                                    <#ty as crate::native::FromDinoRef>::from_dinoref(arg_val, memory)?
                                };
                            });
                        }
                    }
                }
            }
            
            let call_stmt = quote! {
                let result = #self_ty::#method_name( #(#param_names),* );
            };
            
            let param_count = param_names.len();
            let expected_count = if param_count == 0 { 1usize } else { param_count };
            
            let mut push_logic = quote! {};
            if is_symbol {
                push_logic.extend(quote! {
                    let sym_val = crate::types::Symbol::from_name(#sym_name_str).unwrap_or_else(|| panic!("Invalid symbol name: {}", #sym_name_str));
                    _symbols.push((sym_val, _id, 0u8));
                });
            } else {
                push_logic.extend(quote! {
                    _methods.push((#method_key, _id, 0u8));
                });
            }

            let mut push_alias_logic = quote! {};
            if keep_method_name {
                let method_registered_name = format!("{}_{}", struct_name, method_name);
                push_alias_logic.extend(quote! {
                    {
                        let _id = crate::native::register_native_function_with_flags(
                            #method_registered_name,
                            wrapper_alias,
                            1
                        );
                        _methods.push((#method_key_alias, _id, 0u8));
                    }
                });
            }

            let wrapper_fn = quote! {
                let wrapper = move |memory: &mut crate::memory::MemoryManager,
                                   args_start: usize,
                                   args_count: usize|
                      -> crate::errors::Result<crate::types::DinoRef> {
                    
                    if args_count != #expected_count {
                        return Err(crate::errors::RuntimeError::TypeError(
                            format!(
                                "Expected {} arguments, got {}",
                                #expected_count,
                                args_count
                            )
                        ));
                    }
                    
                    #(#arg_processing)*
                    #call_stmt
                    ToDinoRef::to_dinoref(result, memory)
                };
            };

            let alias_wrapper = if keep_method_name {
                quote! {
                    let wrapper_alias = move |memory: &mut crate::memory::MemoryManager,
                                       args_start: usize,
                                       args_count: usize|
                          -> crate::errors::Result<crate::types::DinoRef> {
                        
                        if args_count != #expected_count {
                            return Err(crate::errors::RuntimeError::TypeError(
                                format!(
                                    "Expected {} arguments, got {}",
                                    #expected_count,
                                    args_count
                                )
                            ));
                        }
                        
                        #(#arg_processing)*
                        #call_stmt
                        ToDinoRef::to_dinoref(result, memory)
                    };
                }
            } else { quote! {} };
            
            generated_methods.push(quote! {
                {
                    use crate::native::{FromDinoRef, ToDinoRef};
                    #wrapper_fn
                    let _id = crate::native::register_native_function_with_flags(
                        #registered_name,
                        wrapper,
                        1
                    );
                    #push_logic

                    #alias_wrapper
                    #push_alias_logic
                }
            });
        }
    }
    
    let struct_name_ident: proc_macro2::TokenStream = struct_name.parse().unwrap();
    
    let expanded = quote! {
        #input_impl
        
        paste::paste! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            pub fn [<_dinoclass_init_methods_ #struct_name_ident>]() {
                let mut _methods: Vec<(&'static str, u32, u8)> = Vec::new();
                let mut _symbols: Vec<(crate::types::DinoRef, u32, u8)> = Vec::new();
                #(#generated_methods)*
                let _ = [<_DINOCLASS_METHODS_ #struct_name_ident>].set(_methods);
                let _ = [<_DINOCLASS_SYMBOLS_ #struct_name_ident>].set(_symbols);
            }
        }
    };
    
    TokenStream::from(expanded)
}
