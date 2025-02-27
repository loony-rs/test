use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Validate, attributes(min_length))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let mut field_checks = vec![];

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            for field in fields.named {
                let field_name = field.ident.unwrap();

                for attr in field.attrs {
                    if attr.path().is_ident("min_length") {
                        let meta = attr.parse_meta().unwrap();
                        if let syn::Meta::NameValue(nv) = meta {
                            if let syn::Expr::Lit(lit) = nv.value {
                                if let syn::Lit::Int(len) = lit.lit {
                                    let min_length: usize = len.base10_parse().unwrap();
                                    field_checks.push(quote! {
                                        if self.#field_name.len() < #min_length {
                                            return Err(format!("{} must be at least {} characters long", stringify!(#field_name), #min_length));
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn validate(&self) -> Result<(), String> {
                #(#field_checks)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}