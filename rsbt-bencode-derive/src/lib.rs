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

use proc_macro::{Span, TokenStream};
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, quote_spanned};
use syn::{DataEnum, Lit, Meta, MetaList, MetaNameValue, Path};

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
                    if let Some(input_fn) = field_attribute(&field.attrs, "input")
                        .map(|input_fn|syn::Ident::new(&input_fn, proc_macro2::Span::call_site())) {
                        quote_spanned! { ident.span() => #ident = Some(#input_fn(input)) }
                    } else {
                        let name = field_attribute(&field.attrs, "rename").unwrap_or_else(|| ident.to_string());
                        quote_spanned! { ident.span() => Bencoded::init_fields(&mut parsers, #name, &mut #ident) }
                    }
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
                                Dictionary { entries, input } => {
                                    use #bencode_parse_path::Bencoded;

                                    #(#field_defs)*

                                    {
                                        let mut parsers = Vec::new();

                                        #(#field_parsers;)*

                                        parsers.sort_by_key(|&(key, _)| key);

                                        #bencode_parse_path::parse_bencoded_entries(parsers, entries)?;
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
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let init_fields_parsers = variants.iter().map(|variant| {
                let ident = &variant.ident;
                let name =
                    field_attribute(&variant.attrs, "rename").unwrap_or_else(|| ident.to_string());
                quote_spanned! {
                    ident.span() =>
                        let f_clone = f.clone();
                        parsers.push((
                            #name,
                            Box::new(move |bencode| {
                                let field: Option<_> = Bencoded::try_from_bencoded(bencode)?;
                                f_clone.borrow_mut()(field.map(Self::#ident));
                                Ok(())
                            }),
                        ));
                }
            });
            quote! {
                #[automatically_derived]
                impl #impl_generics #bencode_parse_path::Bencoded<'a> for #name #ty_generics #where_clause {
                    fn init_fields<'c>(
                        parsers: &mut Vec<(
                            &'c str,
                            Box<dyn FnOnce(Bencode<'a>) -> Result<(), BencodeError> + 'c>,
                        )>,
                        _: &'c str,
                        value: &'c mut Option<Self>,
                    ) {
                        let f = Arc::new(RefCell::new(move |field| *value = field));
                        #(#init_fields_parsers)*
                    }

                    fn try_from_bencoded(bencode: #bencode_parse_path::Bencode<'a>) -> Result<Self, #bencode_parse_path::BencodeError> {
                        use #bencode_parse_path::Bencode::*;
                        match bencode {
                            Dictionary { entries, .. } => {
                                use #bencode_parse_path::Bencoded;
                                unimplemented!()
                            }
                            String(_) | Integer(_) | List(_) => Err(#bencode_parse_path::BencodeError::NoMatch),
                        }
                    }
                }
            }
        }
        syn::Data::Union(_) => abort!(name, "unions are not supported"),
    };

    gen.into()
}

fn field_attribute(attrs: &[syn::Attribute], name: &str) -> Option<String> {
    attrs
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
