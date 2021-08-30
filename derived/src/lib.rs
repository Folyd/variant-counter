extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

use crate::{attrs::ParsedAttr, parsed::ParsedEnum};

mod attrs;
mod parsed;

#[proc_macro_derive(VariantCount, attributes(counter))]
pub fn derive_variant_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Enum(data_enum) = &input.data {
        let parsed_attr = match ParsedAttr::parse(&data_enum) {
            Ok(parsed_attr) => parsed_attr,
            Err(error) => return error.into(),
        };

        let parsed = ParsedEnum::parse(&input, data_enum, &parsed_attr);

        let mut quotes = vec![derive_impl(&input, &parsed)];

        if parsed_attr.has_customized_weight() {
            quotes.push(derive_weighted_impl(&input, &parsed));
        }

        TokenStream::from(quote! {
          #(#quotes)*
        })
    } else {
        panic!("VariantCount only works on Enums");
    }
}

fn derive_impl(input: &DeriveInput, parsed: &ParsedEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variant_count = parsed.variant_count;
    let variant_len = parsed.variant_len;
    let match_arm_quotes = &parsed.match_arm_quotes;
    let map_quotes = &parsed.map_quotes;
    let group_map_quotes = &parsed.group_map_quotes;
    let counter_struct = format_ident!("{}Counter", name);

    let check_fns = &parsed.check_quotes;
    let erase_fns = &parsed.erase_quotes;

    quote! {
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
            frequency: [usize; #variant_len],
        }

        impl #counter_struct {
            #vis const fn new() -> #counter_struct {
                #counter_struct {
                    frequency: [0; #variant_len],
                }
            }

            #vis fn record#ty_generics(&mut self, target: &#name#ty_generics) {
                let pair = match target {
                    #(#match_arm_quotes,)*
                    _ => None,
                };

                if let Some(index) = pair {
                    self.frequency[index] = self.frequency[index].saturating_add(1);
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
                    self.frequency[index] = 0;
                }
            }

            #vis fn reset#ty_generics(&mut self) {
                self.frequency = [0; #variant_len];
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

            #[inline]
            #vis fn sum(&self) -> usize {
                self.frequency.iter().sum()
            }

            #[cfg(feature = "stats")]
            #[inline]
            #vis fn avg(&self) -> f64 {
                self.sum() as f64 / #variant_len as f64
            }

            #[cfg(feature = "stats")]
            #[inline]
            #vis fn variance(&self) -> f64 {
                let avg = self.avg();
                self.frequency
                    .iter()
                    .map(|freq| (*freq as f64 - avg).powi(2))
                    .sum::<f64>()
                    / #variant_len as f64
            }

            #[cfg(feature = "stats")]
            #[inline]
            #vis fn sd(&self) -> f64 {
                self.variance().sqrt()
            }
        }
    }
}

fn derive_weighted_impl(input: &DeriveInput, parsed: &ParsedEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = &input.vis;

    let variant_len = parsed.variant_len;
    let weighted_map_quotes = &parsed.weighted_map_quotes;
    let weighted_group_map_quotes = &parsed.weighted_group_map_quotes;
    let counter_struct = format_ident!("{}Counter", name);

    let weight_check_fns = &parsed.weighted_check_quotes;
    let weights = &parsed.weights;
    let weighted_struct = format_ident!("{}Weighted", name);

    quote! {
        impl #counter_struct {
            #vis fn weighted(&self) -> #weighted_struct {
                #weighted_struct::new(&self.frequency)
            }
        }

        #vis struct #weighted_struct<'a> {
            frequency: &'a [usize],
            weight: [usize; #variant_len],
        }

        impl<'a> #weighted_struct<'a> {
            #vis fn new(frequency: &'a [usize]) -> #weighted_struct {
                #weighted_struct {
                    frequency,
                    weight: [#(#weights,)*],
                }
            }

            #[inline]
            #vis fn total_weight(&self) -> usize {
                self.weight.iter().sum()
            }

            #(#weight_check_fns)*

            #vis fn to_map(&self) -> std::collections::HashMap<&'static str, usize> {
                let mut map = std::collections::HashMap::with_capacity(#variant_len);
                #(#weighted_map_quotes)*
                map
            }

            #vis fn to_group_map(&self) -> std::collections::HashMap<&'static str, usize> {
                let mut map = std::collections::HashMap::with_capacity(#variant_len);
                #(#weighted_group_map_quotes)*
                map
            }

            #[inline]
            #vis fn sum(&self) -> usize {
                self.frequency
                    .iter()
                    .zip(self.weight)
                    .map(|(freq, w)| freq * w)
                    .sum()
            }

            #[cfg(feature = "stats")]
            #[inline]
            #vis fn avg(&self) -> f64 {
                self.sum() as f64 / self.total_weight() as f64
            }

            #[cfg(feature = "stats")]
            #[inline]
            #vis fn variance(&self) -> f64 {
                let avg = self.avg();
                self.frequency
                    .iter()
                    .zip(self.weight)
                    .map(|(freq, w)| ((freq * w) as f64 - avg).powi(2))
                    .sum::<f64>() / self.total_weight() as f64
            }

            #[cfg(feature = "stats")]
            #[inline]
            #vis fn sd(&self) -> f64 {
                self.variance().sqrt()
            }
        }
    }
}
