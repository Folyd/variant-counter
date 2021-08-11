extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

struct ParsedEnum {
    variant_len: usize,
    record_quotes: Vec<proc_macro2::TokenStream>,
    map_quotes: Vec<proc_macro2::TokenStream>,
}

#[proc_macro_derive(VariantCount)]
pub fn derive_variant_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let vis = input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let parsed = match input.data {
        Data::Enum(data_enum) => {
            let variant_len = data_enum.variants.len();
            let mut record_quotes = Vec::with_capacity(variant_len);
            let mut map_quotes = Vec::with_capacity(variant_len);
            data_enum
                .variants
                .iter()
                .enumerate()
                .for_each(|(index, variant)| {
                    let variant_name = &variant.ident;
                    let display_variant_name = variant_name.to_string();
                    map_quotes.push(quote! {
                        map.insert(#display_variant_name, self.container[#index]);
                    });
                    match &variant.fields {
                        Fields::Named(_) => {
                            record_quotes.push(quote! {
                                #name::#variant_name{ .. } => #index
                            });
                        }
                        Fields::Unnamed(f) => {
                            if f.unnamed.is_empty() {
                                record_quotes.push(quote! {
                                    #name::#variant_name() => #index
                                });
                            } else {
                                record_quotes.push(quote! {
                                    #name::#variant_name(..) => #index
                                });
                            }
                        }
                        Fields::Unit => record_quotes.push(quote! {
                            #name::#variant_name => #index
                        }),
                    }
                });
            ParsedEnum {
                variant_len,
                record_quotes,
                map_quotes,
            }
        }
        _ => panic!("VariantCount only works on Enums"),
    };

    let variant_count = parsed.variant_len;
    let record_quotes = parsed.record_quotes;
    let map_quotes = parsed.map_quotes;
    let counter_struct = format_ident!("{}Counter", name);
    let expanded = quote! {
        #[derive(Debug)]
        #[must_use]
        #vis struct #counter_struct {
            container: [usize; #variant_count],
        }

        impl #counter_struct {
            #vis fn new() -> #counter_struct {
                #counter_struct { container: [0; #variant_count]  }
            }

            #vis fn record #ty_generics (&mut self, target: &#name#ty_generics) {
                let index = match target {
                    #(#record_quotes,)*
                };

                debug_assert!(index < #variant_count);
                self.container[index] = self.container[index].saturating_add(1);
            }

            #vis fn erase #ty_generics (&mut self, target: &#name#ty_generics) {
                let index = match target {
                    #(#record_quotes,)*
                };

                debug_assert!(index < #variant_count);
                self.container[index] = self.container[index].saturating_sub(1);
            }

            #vis fn to_map(&self) -> std::collections::HashMap<&'static str, usize> {
                let mut map = std::collections::HashMap::with_capacity(#variant_count);
                #(#map_quotes)*
                map
            }
        }

        impl #impl_generics variant_counter::VariantCount for #name #ty_generics #where_clause {
            type Target = #counter_struct;
            fn counter() -> Self::Target {
                #counter_struct::new()
            }
        }

    };

    TokenStream::from(expanded)
}
