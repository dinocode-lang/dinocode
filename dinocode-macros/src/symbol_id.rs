// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/symbol_id.rs
//  Desc:       symbol_id attribute implementation
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemConst};
use std::sync::atomic::{AtomicU32, Ordering};

pub static SYMBOL_ID_COUNTER: AtomicU32 = AtomicU32::new(16);

pub fn symbol_id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_const = parse_macro_input!(item as ItemConst);
    
    let const_name = &input_const.ident;
    let const_vis = &input_const.vis;
    
    let id = SYMBOL_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    
    let expanded = quote! {
        #const_vis const #const_name: crate::types::DinoRef = crate::types::DinoRef::symbol(#id);
    };
    
    TokenStream::from(expanded)
}
