#![feature(proc_macro, proc_macro_lib, custom_derive)]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(EnumFromInner)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let ast = syn::parse_macro_input(&input).unwrap();

    let ref variants = if let syn::Body::Enum(ref e) = ast.body {
        e
    } else {
        panic!("#[derive(EnumFromInner)] can only be used with enums");
    };

    let ref enum_ty = ast.ident;

    let mut froms = vec![];
    for v in variants.iter() {
        if let syn::VariantData::Tuple(ref fields) = v.data {
            let types: Vec<_> = fields.into_iter().map(|f| f.ty.clone()).collect();
            if types.is_empty() { continue; }

            if types.len() == 1 {
                let ref t = types[0];
                let ref id = v.ident;

                froms.push(quote! {
                    impl From<#t> for #enum_ty {
                        fn from(t: #t) -> #enum_ty {
                            #enum_ty::#id(t)
                        }
                    }
                });
            }
        }
    }

    (quote! { #ast #(#froms)* }).to_string().parse().unwrap()
}
