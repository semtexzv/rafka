extern crate proc_macro;

use proc_macro::{TokenStream};

use syn::{DataStruct, Fields, Meta, LitInt, parse_macro_input, DeriveInput};
use quote::{quote, quote_spanned};

use darling::FromMeta;
use syn::spanned::Spanned;
use syn::export::TokenStream2;

#[derive(Debug, darling::FromMeta)]
struct Wrapper {
    #[darling(default)]
    name: String,
    #[darling(default)]
    since: Option<usize>,
    #[darling(default)]
    until: Option<usize>,
    #[darling(default)]
    default: bool,
}

#[derive(Debug, darling::FromMeta)]
struct StructArgs {}

#[derive(Debug, darling::FromMeta)]
struct FieldArgs {
    #[darling(default)]
    since: Option<usize>,
    #[darling(default)]
    until: Option<usize>,
    #[darling(default)]
    tag_since: Option<usize>,

    #[darling(default)]
    tags: bool,
}

fn args_from_attr<T: FromMeta>(attr: &syn::Attribute) -> Option<T> {
    if attr.path.get_ident().unwrap() == "wired" {
        let data: Meta = attr.parse_meta().expect("wired attribute args");
        let args = T::from_meta(&data).expect("`wired` attr arguments");
        Some(args)
    } else {
        None
    }
}


fn de(wrap: TokenStream2) -> (TokenStream2, TokenStream2) {
    let span = wrap.span().clone();
    (
        quote_spanned! { span => crate::proto::Wired::from_wire(wire)? },
        quote_spanned! { span => crate::proto::Wired::from_wire_compact(wire)? }
    )
}


fn ser(get: TokenStream2) -> (TokenStream2, TokenStream2) {
    let span = get.span().clone();
    (
        quote_spanned! { span => crate::proto::Wired::to_wire(#get, wire); },
        quote_spanned! { span => crate::proto::Wired::to_wire_compact(#get, wire); }
    )
}

fn opt_to_from(cond: &TokenStream2, to: &TokenStream2, from: &TokenStream2) -> (TokenStream2, TokenStream2) {
    let span = cond.span().clone();

    (
        quote_spanned! { span => if #cond { #to } },
        quote_spanned! { span => if #cond { Some(#from) } else { None }}
    )
}

#[proc_macro_derive(Wired, attributes(wired))]
pub fn derive_wired(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (to, from, to_compact, from_compact) = match input.data {
        syn::Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
            // TODO: Use for specifying tagged_since
            let mut _args = input.attrs.iter().filter_map(args_from_attr::<StructArgs>);

            let versions = fields.named.iter().map(|f| {
                let vals = f.attrs.iter().filter_map(args_from_attr::<FieldArgs>);
                if let Some(v) = vals.last() {
                    return Some((v.since, v.until));
                }
                return None;
            });

            let names = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote_spanned! {f.ident.span() => #name}
            });

            let (names, n1): (Vec<_>, Vec<_>) = names.map(|n| (n.clone(), n)).unzip();

            let merged = versions.zip(n1).map(|(ver, name)| {
                let to_from: (TokenStream2, TokenStream2);
                let to_from_compact: (TokenStream2, TokenStream2);

                let (to_opt, to_opt_compact) = ser(quote_spanned! { name.span() => self.#name.as_ref().expect("VER")});
                let (from_opt, from_opt_compact) = de(quote_spanned! { name.span() => Some });

                match ver {
                    Some((Some(since), None)) => {
                        let since = LitInt::new(&since.to_string(), name.span());
                        let cond = quote_spanned! { name.span() => wire.version >= #since };
                        to_from = opt_to_from(&cond, &to_opt, &from_opt);
                        to_from_compact = opt_to_from(&cond, &to_opt_compact, &from_opt_compact);
                    }
                    Some((None, Some(until))) => {
                        let until = LitInt::new(&until.to_string(), name.span());
                        let cond = quote_spanned! { name.span() => wire.version <= #until };
                        to_from = opt_to_from(&cond, &to_opt, &from_opt);
                        to_from_compact = opt_to_from(&cond, &to_opt_compact, &from_opt_compact);
                    }
                    Some((Some(since), Some(until))) => {
                        let since = LitInt::new(&since.to_string(), name.span());
                        let until = LitInt::new(&until.to_string(), name.span());
                        let cond = quote_spanned! { name.span() => wire.version >= #since && wire.version <= #until };

                        to_from = opt_to_from(&cond, &to_opt, &from_opt);
                        to_from_compact = opt_to_from(&cond, &to_opt_compact, &from_opt_compact);
                    }
                    Some((None, None)) | None => {
                        to_from = (
                            quote_spanned! {name.span() => self.#name.to_wire(wire); },
                            quote_spanned! {name.span() => crate::proto::Wired::from_wire(wire)?}
                        );
                        to_from_compact = (
                            quote_spanned! {name.span() => self.#name.to_wire_compact(wire); },
                            quote_spanned! {name.span() => crate::proto::Wired::from_wire_compact(wire)?}
                        );
                    }
                }
                (to_from, to_from_compact)
            });

            let (to_from, to_from_compact): (Vec<_>, Vec<_>) = merged.unzip();

            let (to, from): (Vec<_>, Vec<_>) = to_from.into_iter().unzip();
            let (to_compact, from_compact): (Vec<_>, Vec<_>) = to_from_compact.into_iter().unzip();


            (
                quote! { #(#to)* },
                quote! { Ok(Self { #(#names: #from,)* })},
                quote! { #(#to_compact)* },
                quote! { Ok(Self{#(#names: #from_compact,)*})}
            )
        }
        _ => {
            panic!("Unsupported")
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics crate::proto::Wired for #ident #ty_generics #where_clause {
            fn to_wire(&self, wire: &mut crate::proto::WireWrite) {
                #to
            }
            fn from_wire(wire: &mut crate::proto::WireRead) -> Result<Self, crate::proto::Error> {
                #from
            }
            fn to_wire_compact(&self, wire: &mut crate::proto::WireWrite) {
                #to_compact
            }
            fn from_wire_compact(wire: &mut crate::proto::WireRead) -> Result<Self, crate::proto::Error> {
                #from_compact
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