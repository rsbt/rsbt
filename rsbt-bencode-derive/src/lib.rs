/*!
# rsbt-bencode-derive description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-bencode-derive = "0.1"
```

*/

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, quote_spanned};
use syn::{Lit, Meta, MetaList, MetaNameValue, Path};

#[proc_macro_derive(BencodeParse, attributes(bencode))]
#[proc_macro_error]
pub fn bencode_parse_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_bencode_parse(&ast)
}

fn impl_bencode_parse(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let bencode_parse_path = quote! { ::rsbt_bencode_nom7 };

    let data = &ast.data;

    let gen = match data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                let mut field_ids: Vec<_> = fields_named
                    .named
                    .iter()
                    .filter(|f| f.ident.is_some())
                    .collect();

                field_ids.sort_by_key(|k| k.ident.as_ref());

                let field_parsers = field_ids.iter().map(|field| {
                    let ident = field.ident.as_ref().unwrap();
                    let name = attribute(field, "rename").unwrap_or_else(|| ident.to_string());
                    quote_spanned! { ident.span() => Bencoded::field_parsers(#name, |value| { #ident = value; }) }
                });

                let fields = field_ids
                    .iter()
                    .filter_map(|f| f.ident.as_ref())
                    .map(|field| quote! { #field });

                let field_defs = field_ids
                    .iter()
                    .filter_map(|f| f.ident.as_ref())
                    .map(|field| quote! { let mut #field = None; });

                quote! {
                    #[automatically_derived]
                    impl #impl_generics #bencode_parse_path::Bencoded<'a> for #name #ty_generics #where_clause {

                        fn try_from_bencoded(bencode: #bencode_parse_path::Bencode<'a>) -> Result<Self, #bencode_parse_path::BencodeError> {
                            use #bencode_parse_path::Bencode::*;
                            match bencode {
                                Dictionary(entries) => {
                                    use rsbt_bencode_nom7::Bencoded;

                                    #(#field_defs)*

                                    let mut parsers = Vec::new();
                                    #(parsers.push(#field_parsers);)*

                                    let mut parsers: Vec<_> = parsers.into_iter().flatten().collect();
                                    parsers.sort_by_key(|&(key, _)| key);

                                    let mut entries = entries.into_iter();

                                    for (_, parser_fn) in parsers {
                                        entries = parser_fn(entries)?;
                                    }

                                    Ok(Self {
                                        #(#fields: #fields.unwrap() ,)*
                                    })
                                }
                                String(_) | Integer(_) | List(_) => Err(#bencode_parse_path::BencodeError::NoMatch),
                            }
                        }
                    }
                }
            }
            syn::Fields::Unnamed(_) => {
                abort!(data_struct.fields, "unnamed fields are not supported")
            }
            syn::Fields::Unit => abort!(name, "unit fields are not supported"),
        },
        syn::Data::Enum(_) => abort!(name, "enums are not supported"),
        syn::Data::Union(_) => abort!(name, "unions are not supported"),
    };

    gen.into()
}

fn attribute(f: &syn::Field, name: &str) -> Option<String> {
    f.attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| match meta {
            syn::Meta::List(MetaList {
                path: Path { segments, .. },
                nested,
                ..
            }) if segments.len() == 1 && segments[0].ident == "bencode" => Some(nested),
            _ => None,
        })
        .filter_map(|nested| {
            nested
                .iter()
                .filter_map(|nested_meta| match nested_meta {
                    syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        path: Path { segments, .. },
                        lit: Lit::Str(lit),
                        ..
                    })) if segments.len() == 1 && segments[0].ident == name => Some(lit.value()),
                    _ => None,
                })
                .next()
        })
        .next()
}
