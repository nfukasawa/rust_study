extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Deserialize)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields: Vec<(&syn::Ident, &syn::Ident)> = match ast.data {
        syn::Data::Struct(ref s) => s
            .fields
            .iter()
            .filter_map(|field| match (&field.ident, type_ident(field)) {
                (Some(name), Some(ty)) => Some((name, ty)),
                _ => None,
            })
            .collect(),
        syn::Data::Enum(_) => unreachable!(),
        syn::Data::Union(_) => unreachable!(),
    };

    println!("fields: {:?}", fields);

    let tokens = quote! {
        impl #name {
            fn foo(&self) {
                println!("foo");
            }
        }
    };
    TokenStream::from(tokens)
}

fn type_ident(field: &syn::Field) -> Option<&syn::Ident> {
    match &field.ty {
        syn::Type::Path(p) => match p.path.segments.first() {
            Some(seg) => Some(&seg.ident),
            _ => None,
        },
        _ => None, // TODO
    }
}
