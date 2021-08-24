extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

use crate::attrs::ParsedAttr;

mod attrs;

struct ParsedEnum {
    // The number of variants in the enum type.
    variant_count: usize,
    // The number of variants excluding ignored in the enum type.
    variant_len: usize,
    match_arm_quotes: Vec<proc_macro2::TokenStream>,
    weights: Vec<proc_macro2::TokenStream>,
    #[cfg(feature = "check")]
    check_quotes: Vec<proc_macro2::TokenStream>,
    #[cfg(feature = "erase")]
    erase_quotes: Vec<proc_macro2::TokenStream>,
    map_quotes: Vec<proc_macro2::TokenStream>,
    group_map_quotes: Vec<proc_macro2::TokenStream>,
}

#[proc_macro_derive(VariantCount, attributes(counter))]
pub fn derive_variant_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let parsed = match &input.data {
        Data::Enum(data_enum) => {
            let parsed_attr = match ParsedAttr::parse(&data_enum) {
                Ok(parsed_attr) => parsed_attr,
                Err(error) => return error.into(),
            };

            let variant_count = data_enum.variants.len();
            let variant_len = variant_count - parsed_attr.ignores.len();
            let mut weights = Vec::with_capacity(variant_len);
            let mut check_quotes = Vec::with_capacity(variant_len);
            let mut erase_quotes = Vec::with_capacity(variant_len);
            let mut match_arm_quotes = Vec::with_capacity(variant_len);
            let mut map_quotes = Vec::with_capacity(variant_len);
            let variant_index_map = data_enum
                .variants
                .iter()
                .filter(|variant| !parsed_attr.is_ignored(&variant))
                .enumerate()
                .map(|(index, variant)| (&variant.ident, index))
                .collect::<HashMap<&proc_macro2::Ident, usize>>();

            data_enum
                .variants
                .iter()
                .filter(|variant| !parsed_attr.is_ignored(&variant))
                .for_each(|variant| {
                    let variant_name = &variant.ident;
                    let index = variant_index_map[variant_name];
                    let display_variant_name = variant_name.to_string();

                    let check_fn_name =
                        format_ident!("check_{}", display_variant_name.to_lowercase());
                    check_quotes.push(quote! {
                        #[inline]
                        #vis const fn #check_fn_name(&self) -> usize {
                            self.container[#index]
                        }
                    });
                    map_quotes.push(quote! {
                        map.insert(#display_variant_name, self.container[#index]);
                    });

                    match &variant.fields {
                        Fields::Named(_) => {
                            match_arm_quotes.push(quote! {
                                #name::#variant_name{ .. } => Some(#index)
                            });
                        }
                        Fields::Unnamed(f) => {
                            if f.unnamed.is_empty() {
                                match_arm_quotes.push(quote! {
                                    #name::#variant_name() => Some(#index)
                                });
                            } else {
                                match_arm_quotes.push(quote! {
                                    #name::#variant_name(..) => Some(#index)
                                });
                            }
                        }
                        Fields::Unit => match_arm_quotes.push(quote! {
                            #name::#variant_name => Some(#index)
                        }),
                    }

                    let weight = parsed_attr.weight.get(&variant_name).copied().unwrap_or(1);
                    weights.push(parsed_attr.weight.get(&variant_name).copied().unwrap_or(1));

                    let erase_fn_name =
                        format_ident!("erase_{}", display_variant_name.to_lowercase());
                    erase_quotes.push(quote! {
                        #[inline]
                        #vis fn #erase_fn_name(&mut self) {
                            self.container[#index] = self.container[#index].saturating_sub(#weight);
                        }
                    });
                });
            ParsedEnum {
                variant_count,
                variant_len,
                weights: weights
                    .into_iter()
                    .map(|weight| quote! { #weight })
                    .collect(),
                match_arm_quotes,
                #[cfg(feature = "check")]
                check_quotes,
                #[cfg(feature = "erase")]
                erase_quotes,
                map_quotes,
                group_map_quotes: parsed_attr
                    .groups
                    .iter()
                    .map(|(group_name, idents)| {
                        let variant_quotes = idents
                            .iter()
                            .filter_map(|ident| variant_index_map.get(ident))
                            .map(|index| quote! { self.container[#index] })
                            .collect::<Vec<proc_macro2::TokenStream>>();
                        quote! {
                            map.insert(#group_name, #(#variant_quotes)+*);
                        }
                    })
                    .collect(),
            }
        }
        _ => panic!("VariantCount only works on Enums"),
    };

    let variant_count = parsed.variant_count;
    let variant_len = parsed.variant_len;
    let match_arm_quotes = &parsed.match_arm_quotes;
    let map_quotes = parsed.map_quotes;
    let group_map_quotes = parsed.group_map_quotes;
    let counter_struct = format_ident!("{}Counter", name);
    let stats_struct = format_ident!("{}Stats", name);

    #[cfg(feature = "check")]
    let check_fns = parsed.check_quotes;
    #[cfg(not(feature = "check"))]
    let check_fns = vec![quote! {}];

    #[cfg(feature = "check")]
    let erase_fns = parsed.erase_quotes;
    #[cfg(not(feature = "check"))]
    let erase_fns = vec![quote! {}];

    let weights = parsed.weights;

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[inline]
            fn variant_count() -> usize {
                #variant_count
            }
        }

        impl #impl_generics variant_counter::VariantCount for #name #ty_generics #where_clause {
            type Target = #counter_struct;

            fn counter() -> Self::Target {
                #counter_struct::new()
            }
        }

        #[derive(Debug, Clone, Copy)]
        #[must_use]
        #vis struct #counter_struct {
            container: [usize; #variant_len],
            weights: [usize; #variant_len],
        }

        impl #counter_struct {
            #vis const fn new() -> #counter_struct {
                #counter_struct {
                    container: [0; #variant_len],
                    weights: [#(#weights,)*],
                }
            }

            #vis fn record#ty_generics(&mut self, target: &#name#ty_generics) {
                let pair = match target {
                    #(#match_arm_quotes,)*
                    _ => None,
                };

                if let Some(index) = pair {
                    self.container[index] = self.container[index].saturating_add(self.weights[index]);
                }
            }

            #(#erase_fns)*

            #(#check_fns)*

            #vis fn discard#ty_generics(&mut self, target: &#name#ty_generics) {
                let index = match target {
                    #(#match_arm_quotes,)*
                    _ => None,
                };

                if let Some(index) = index {
                    self.container[index] = 0;
                }
            }

            #vis fn reset#ty_generics(&mut self) {
                self.container = [0; #variant_len];
            }

            #vis fn to_map(&self) -> std::collections::HashMap<&'static str, usize> {
                let mut map = std::collections::HashMap::with_capacity(#variant_len);
                #(#map_quotes)*
                map
            }

            #vis fn to_group_map(&self) -> std::collections::HashMap<&'static str, usize> {
                let mut map = std::collections::HashMap::with_capacity(#variant_len);
                #(#group_map_quotes)*
                map
            }

            #vis fn stats(&self) -> #stats_struct {
                #stats_struct::new(*self)
            }
        }

        #vis struct #stats_struct {
            counter: #counter_struct,
        }

        impl #stats_struct {
            #vis fn new(counter: #counter_struct) -> #stats_struct {
                #stats_struct { counter }
            }

            #[inline]
            #vis fn sum(&self) -> usize {
                self.counter.container.iter().sum()
            }

            #[inline]
            #vis fn avg(&self) -> usize {
                self.sum() / #variant_len
            }
        }
    };

    TokenStream::from(expanded)
}
