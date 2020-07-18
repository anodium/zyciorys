use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::TokenStreamExt;
use proc_macro2::TokenTree;
use syn::{Attribute, Data, DeriveInput, Field, Expr, ExprField, Fields, Ident, parse_macro_input, punctuated::Pair, punctuated::Pairs, punctuated::Punctuated, token::Comma, PathSegment};

#[proc_macro_derive(YamlPubKeys)]
pub fn with_pub_keys(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(YamlKeyed, attributes(keyed))]
pub fn derive_keyed_helper(_item: TokenStream) -> TokenStream {
    let mut output: TokenStream2 = TokenStream2::new();

    println!("NOTE: _ITEM::: {:#?}", _item);

    let input = parse_macro_input!(_item as DeriveInput);
    println!("NOTE: INPUT::: {:#?}", input);

    let ir0 = input.data;
    println!("NOTE: IR0::: {:#?}", ir0);

    let ir1 = match ir0 {
        Data::Struct(d) => d,
        _ => panic!()
    };
    println!("NOTE: IR1::: {:#?}", ir1);

    let ir2 = match ir1.fields {
        Fields::Named(f) => f,
        _ => panic!()
    };
    println!("NOTE: IR2::: {:#?}", ir2);

    let ir3 = ir2.named;
    println!("NOTE: IR3::: {:#?}", ir3);

    for pair in ir3.pairs() {
        let tuple: (&Field, Option<&Comma>) = pair.into_tuple();
        println!("NOTE: PAIR:::: {:#?} :: {:#?} ;;", tuple.0, tuple.1);

        let name: Ident = tuple.0.ident.clone().unwrap();

        let mut has_attribute: bool = false;
        for attr in &tuple.0.attrs {
            for segment in &attr.path.segments {
                let ident = &segment.ident;
                if ident.to_string() == "keyed" {
                    has_attribute = true;
                    println!("NOTE: FOUND ATTRIBUTE IN FIELD: {:#?}", &name);
                }
            }
        }

        if has_attribute {
            let tree: TokenStream2 = quote! (
                pub const __KEYED_#{str::String::to_upper(name.to_string())}
            );

            // FIXME: does the conversion go TokenStream2 -> TokenTree -> match() -> TokenTree::Variant??
            /*output.append(match tree {
                proc_macro2::
            });*/
        }
    }

    println!("NOTE: OUTPUT::: {:#?}", &output);

    TokenStream::from(output)
}
