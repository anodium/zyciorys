use proc_macro::TokenStream;

#[allow(unused_variables)]
#[proc_macro_attribute]
pub fn nop(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
