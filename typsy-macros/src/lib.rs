use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse, spanned::Spanned};

struct Name {
    crate_path: syn::Path,
    name: syn::Ident,
}

impl parse::Parse for Name {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let crate_path = input.parse()?;
        let name = input.parse()?;
        Ok(Self { crate_path, name })
    }
}

#[proc_macro]
pub fn name(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    make_name(syn::parse_macro_input!(tokens as Name)).into()
}

fn make_name(input: Name) -> TokenStream {
    let crate_path = input.crate_path;
    let span = input.name.span();
    let mut buffer = [0; 4];

    let name = input.name.to_string();

    if name.starts_with('_') {
        if let Ok(_) = name[1..].parse::<u64>() {
            let name = &input.name;
            return quote!(#crate_path::anon::character::#name)
        }
    }

    let name = name.chars().map(|c| {
        let c = c.encode_utf8(&mut buffer);
        syn::Ident::new(c, span)
    });

    name.rev().fold(
        quote!(#crate_path::hlist::Nil),
        |output, c| quote!(#crate_path::hlist::Cons<#crate_path::anon::character::#c, #output>),
    )
}

#[proc_macro_derive(Transform)]
pub fn transform(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput {
        data, generics, ident, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let syn::DataStruct { fields, .. } = match data {
        syn::Data::Struct(data) => data,
        _ => return quote_spanned!(ident.span() => compile_error!{"You cannot transform an `enum` or `union`"}).into(),
    };

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut errors = quote!();

    for field in &fields {
        match field.vis {
            syn::Visibility::Public(_) => (),
            _ => errors.extend(
                quote_spanned!(field.ident.as_ref().map(syn::Ident::span).unwrap_or(field.ty.span()) => compile_error!{"All fields must be `pub`"}),
            ),
        }
    }

    let output = match fields {
        syn::Fields::Named(fields) => {
            let fields = fields.named;

            let names = fields.iter().map(|name| {
                make_name(Name {
                    crate_path: syn::parse_quote!(typsy),
                    name: name.ident.as_ref().unwrap().clone(),
                })
            });
            let field_names = fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
            let canon = fields.iter().map(|syn::Field { ident, ty, .. }| quote!(#ident: #ty));

            quote!(
                #errors
                impl #impl_generics typsy::anon::Transform for #ident #type_generics #where_clause {
                    type Canon = typsy::Anon!(#(#canon),*);

                    fn from_canon(canon: Self::Canon) -> Self {
                        #(let typsy::hlist::Cons { value: #field_names, rest: canon } = canon;)*
                        let typsy::hlist::Nil = canon;
                        Self { #(#field_names: #field_names.0),* }
                    }

                    fn into_canon(self) -> Self::Canon {
                        let Self { #(#field_names),* } = self;
                        typsy::anon!(#(#field_names = #field_names),*)
                    }
                }
            )
        }
        syn::Fields::Unnamed(fields) => {
            let fields = fields.unnamed;
            let canon = fields.iter().map(|syn::Field { ty, .. }| ty);
            let mut buffer = String::new();
            let field_names = fields
                .iter()
                .enumerate()
                .map(|(x, field)| {
                    use std::fmt::Write;
                    buffer.clear();
                    write!(&mut buffer, "_{}", x).unwrap();
                    syn::Ident::new(&buffer, field.ty.span())
                })
                .collect::<Vec<_>>();
            quote!(
                #errors
                impl #impl_generics typsy::anon::Transform for #ident #type_generics #where_clause {
                    type Canon = typsy::Anon!(#(#canon),*);

                    fn from_canon(canon: Self::Canon) -> Self {
                        #(let typsy::hlist::Cons { value: #field_names, rest: canon } = canon;)*
                        let typsy::hlist::Nil = canon;
                        Self(#(#field_names.0),*)
                    }

                    fn into_canon(self) -> Self::Canon {
                        let Self(#(#field_names),*) = self;
                        typsy::anon!(#(#field_names),*)
                    }
                }
            )
        }
        syn::Fields::Unit => {
            todo!()
        }
    };

    // println!("{}", output);

    output.into()
}