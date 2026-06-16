// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/dinof.rs
//  Desc:       dinof function macro implementation
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use syn::parse::Parser;

pub fn dinof(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_attrs = &input_fn.attrs;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    
    let params: Vec<_> = input_fn.sig.inputs.iter().collect();
    let param_names: Vec<_> = params.iter().filter_map(|param| {
        if let syn::FnArg::Typed(pat_type) = param {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                Some(&ident.ident)
            } else {
                None
            }
        } else {
            None
        }
    }).collect();
    
    let param_types: Vec<_> = params.iter().filter_map(|param| {
        if let syn::FnArg::Typed(pat_type) = param {
            Some(&pat_type.ty)
        } else {
            None
        }
    }).collect();
    
    let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;
    let args = parser.parse(_attr).expect("Failed to parse attributes");
    let is_raw = args.iter().any(|arg| {
        if let syn::Meta::Path(path) = arg {
            path.is_ident("raw")
        } else {
            false
        }
    });


    let expanded = if is_raw {
        quote! {
            #(#fn_attrs)*
            #fn_vis #fn_sig #fn_block
            
            paste::paste! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub fn [<_dinof_init_ #fn_name>]() {
                    let mut fn_name_str = stringify!(#fn_name);
                    fn_name_str = fn_name_str.strip_prefix("r#").unwrap_or(fn_name_str);
                    
                    let _ = crate::native::register_native_function(
                        fn_name_str,
                        #fn_name
                    );
                }
            }
        }
    } else if param_names.is_empty() {
        quote! {
            #(#fn_attrs)*
            #fn_vis #fn_sig #fn_block
            
            paste::paste! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                fn [<_dinof_init_ #fn_name>]() {
                    use crate::native::ToDinoRef;
                    
                    let wrapper = move |memory: &mut crate::memory::MemoryManager,
                                       _args_start: usize,
                                       _args_count: usize|
                          -> crate::errors::Result<crate::types::DinoRef> {
                        let result = #fn_name();
                        ToDinoRef::to_dinoref(result, memory)
                    };

                    let mut fn_name_str = stringify!(#fn_name);
                    fn_name_str = fn_name_str.strip_prefix("r#").unwrap_or(fn_name_str);
                    
                    let _ = crate::native::register_native_function(
                        fn_name_str,
                        wrapper
                    );
                }
            }
        }
    } else {
        let indices: Vec<usize> = (0..params.len()).collect();

        quote! {
            #(#fn_attrs)*
            #fn_vis #fn_sig #fn_block
            
            paste::paste! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                fn [<_dinof_init_ #fn_name>]() {
                    use crate::native::{FromDinoRef, ToDinoRef};
                    
                    let wrapper = move |memory: &mut crate::memory::MemoryManager,
                                       args_start: usize,
                                       args_count: usize|
                          -> crate::errors::Result<crate::types::DinoRef> {
                        
                        let expected_count = [#(stringify!(#param_names)),*].len();
                        
                        if args_count != expected_count {
                            return Err(crate::errors::RuntimeError::TypeError(
                                format!(
                                    "Expected {} arguments, got {}",
                                    expected_count,
                                    args_count
                                )
                            ));
                        }
                        
                        #(
                            let #param_names = {
                                let arg_val = memory.stack()[args_start + #indices];
                                <#param_types as FromDinoRef>::from_dinoref(
                                    arg_val,
                                    memory
                                )?
                            };
                        )*
                        
                        let result = #fn_name(#(#param_names),*);
                        
                        ToDinoRef::to_dinoref(result, memory)
                    };
                    
                    let mut fn_name_str = stringify!(#fn_name);
                    fn_name_str = fn_name_str.strip_prefix("r#").unwrap_or(fn_name_str);
                    
                    let _ = crate::native::register_native_function(
                        fn_name_str,
                        wrapper
                    );
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
