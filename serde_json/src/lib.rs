extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(Deserialize)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();

    let fields: Vec<(&syn::Ident, &syn::Ident)> = match ast.body {
        syn::Body::Struct(ref vars) => vars
            .fields()
            .iter()
            .filter_map(|field| match (&field.ident, type_ident(field)) {
                (Some(name), Some(ty)) => Some((name, ty)),
                _ => None,
            })
            .collect(),
        syn::Body::Enum(_) => unreachable!(),
    };

    println!("fields: {:?}", fields);

    let output = quote! {
        // TODO
    };
    output.into()
}

fn type_ident(field: &syn::Field) -> Option<&syn::Ident> {
    match &field.ty {
        syn::Ty::Path(_, p) => match p.segments.first() {
            Some(seg) => Some(&seg.ident),
            _ => None,
        },
        _ => None, // TODO
    }
}
