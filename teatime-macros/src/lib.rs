use proc_macro::TokenStream;
use quote::quote;
use syn::{parenthesized, LitStr};

#[proc_macro_derive(QueryParams, attributes(query_params))]
pub fn derive_query_params(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("QueryParams can only be derived for structs"),
    };
    let fields = match data.fields {
        syn::Fields::Named(fields) => fields.named,
        _ => panic!("QueryParams can only be derived for structs with named fields"),
    };

    let field_code = fields.iter().filter_map(|field| {
        let attr = parse_query_params_attr(field);
        if attr.skip {
            return None;
        }
        let field_name = field.ident.as_ref().expect("Field must have an identifier");
        let param = attr.rename.unwrap_or(field_name.to_string());
        Some(quote! {
            if let Some(#field_name) = &self.#field_name {
                params.append_pair(#param, &#field_name.to_string());
            }
        })
    });

    quote! {
        impl #generics #name #generics {
            pub fn append_query_params(&self, req: &mut ::reqwest::Request) {
                let mut params = req.url_mut().query_pairs_mut();
                #(#field_code)*
            }
        }
    }
    .into()
}

#[derive(Default)]
struct QueryParamsAttr {
    skip: bool,
    rename: Option<String>,
}

fn parse_query_params_attr(field: &syn::Field) -> QueryParamsAttr {
    let skip = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("query_params"));
    let mut result = QueryParamsAttr::default();
    if let Some(attr) = skip {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                result.skip = true;
            } else if meta.path.is_ident("rename") {
                let content = meta.value().expect("Expected a value");
                let lit: LitStr = content.parse()?;
                result.rename = Some(lit.value());
            }
            Ok(())
        })
        .expect("Failed to parse query_params attribute");
    }
    result
}
