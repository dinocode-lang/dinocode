use proc_macro::TokenStream;

mod attributes;
mod dinoclass;
mod dinof;
mod dinomethods;
mod symbol_id;

#[proc_macro_attribute]
pub fn dinof(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dinof::dinof(_attr, item)
}

#[proc_macro_attribute]
pub fn dinoclass(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dinoclass::dinoclass(_attr, item)
}

#[proc_macro_attribute]
pub fn raw(_attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::raw(_attr, item)
}

#[proc_macro_attribute]
pub fn getter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::getter(_attr, item)
}

#[proc_macro_attribute]
pub fn setter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::setter(_attr, item)
}

#[proc_macro_attribute]
pub fn dinomethods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dinomethods::dinomethods(_attr, item)
}

#[proc_macro_attribute]
pub fn dinomethod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dinomethods::dinomethod(_attr, item)
}

#[proc_macro_attribute]
pub fn symbol(_attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::symbol(_attr, item)
}

#[proc_macro_attribute]
pub fn symbol_id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    symbol_id::symbol_id(_attr, item)
}
