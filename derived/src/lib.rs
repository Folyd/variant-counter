extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

struct ParsedEnum {
    variant_len: usize,
    variant_quotes: Vec<proc_macro2::TokenStream>,
}

#[proc_macro_derive(VariantCount)]
pub fn derive_variant_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let parsed = match input.data {
        Data::Enum(data_enum) => {
            let variant_quotes = data_enum
                .variants
                .iter()
                .enumerate()
                .map(|(index, variant)| {
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Named(_) => quote! {
                            #name::#variant_name{ .. } => self.increase(#index)
                        },
                        Fields::Unnamed(f) => {
                            if f.unnamed.is_empty() {
                                quote! {
                                    #name::#variant_name() => self.increase(#index)
                                }
                            } else {
                                quote! {
                                    #name::#variant_name(_) => self.increase(#index)
                                }
                            }
                        }
                        Fields::Unit => quote! {
                            #name::#variant_name => self.increase(#index)
                        },
                    }
                })
                .collect();
            ParsedEnum {
                variant_len: data_enum.variants.len(),
                variant_quotes,
            }
        }
        _ => panic!("VariantCount only works on Enums"),
    };

    let variant_count = parsed.variant_len;
    let variant_quotes = parsed.variant_quotes;
    let counter_struct = format_ident!("{}Counter", name);
    let expanded = quote! {
        #[derive(Debug)]
        #[must_use]
        pub struct #counter_struct {
            container: [usize; #variant_count],
        }

        impl #counter_struct {
            pub fn new() -> #counter_struct {
                #counter_struct { container: [0; #variant_count]  }
            }

            #[inline]
            fn increase(&mut self, index: usize) {
                debug_assert!(index < #variant_count);
                self.container[index] += 1;
            }

            fn record(&mut self, target: &#name) {
                match target {
                    #(#variant_quotes,)*
                }
            }
        }

        impl variant_counter::VariantCount for #name {
            type Target = #counter_struct;
            fn counter(&self) -> Self::Target {
                #counter_struct::new()
            }
        }

    };

    TokenStream::from(expanded)
}
