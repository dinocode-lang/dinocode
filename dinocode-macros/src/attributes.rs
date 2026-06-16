// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/attributes.rs
//  Desc:       Simple attribute macros (raw, getter, setter)
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use proc_macro::TokenStream;

pub fn raw(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

pub fn getter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

pub fn setter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

pub fn symbol(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
