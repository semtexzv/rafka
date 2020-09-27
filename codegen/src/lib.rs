extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{DataStruct, Fields, parse_macro_input, DeriveInput};
use quote::quote;


/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Wired)]
pub fn derive_wired(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (fields, fields_read) = match input.data {
        syn::Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
            let names = fields.named.iter().map(|f| &f.ident);
            let names2 = fields.named.iter().map(|f| &f.ident);
            (quote! { #(self.#names.to_wire(to);)* },
             quote! { Ok(Self {
                #(#names2: crate::proto::Wired::from_wire(wire)?,)*
            })})
        }
        syn::Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
            let idxs = fields.unnamed.iter().enumerate().map(|(p, _)| p);

            let reads = fields.unnamed.iter()
                .enumerate()
                .map(|_| quote! {
                    crate::proto::Wired::from_wire(wire)?
                });


            (quote! { #(self.#idxs.to_wire(to);)*}, quote! { #(#reads,)*})
        }
        _ => {
            panic!("Unsupported")
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics crate::proto::Wired for #ident #ty_generics #where_clause {
            fn to_wire(&self, to: &mut crate::proto::WireWrite) {
                #fields
            }
            fn from_wire(wire: &mut crate::proto::WireRead) -> Result<Self, crate::proto::Error> {
                #fields_read
            }
        }
    };

    tokens.into()
}

/*
#[proc_macro_derive(Wired)]
pub fn derive_apitypes(input: TokenStream) -> TokenStream {

}
 */