use std::collections::HashMap;

use quote::{format_ident, quote};
use syn::{DataEnum, DeriveInput, Fields};

use crate::attrs::ParsedAttr;

pub(crate) struct ParsedEnum {
    // The number of variants in the enum type.
    pub(crate) variant_count: usize,
    // The number of variants excluding ignored in the enum type.
    pub(crate) variant_len: usize,
    pub(crate) match_arm_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) weights: Vec<proc_macro2::TokenStream>,
    pub(crate) check_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) weighted_check_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) erase_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) aggregate_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) group_aggregate_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) weighted_aggregate_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) weighted_group_aggregate_quotes: Vec<proc_macro2::TokenStream>,
    pub(crate) has_customized_group: bool,
}

impl ParsedEnum {
    pub(crate) fn parse(
        input: &DeriveInput,
        data_enum: &DataEnum,
        parsed_attr: &ParsedAttr,
    ) -> Self {
        let name = &input.ident;
        let vis = &input.vis;
        let variant_count = data_enum.variants.len();
        let variant_len = variant_count - parsed_attr.ignores.len();
        let mut weights = Vec::with_capacity(variant_len);
        let mut check_quotes = Vec::with_capacity(variant_len);
        let mut weighted_check_quotes = Vec::with_capacity(variant_len);
        let mut erase_quotes = Vec::with_capacity(variant_len);
        let mut match_arm_quotes = Vec::with_capacity(variant_len);
        let mut aggregate_quotes = Vec::with_capacity(variant_len);
        let mut weighted_aggregate_quotes = Vec::with_capacity(variant_len);
        let variant_index_map = data_enum
            .variants
            .iter()
            .filter(|variant| !parsed_attr.is_ignored(variant))
            .enumerate()
            .map(|(index, variant)| (&variant.ident, index))
            .collect::<HashMap<&proc_macro2::Ident, usize>>();

        data_enum
            .variants
            .iter()
            .filter(|variant| !parsed_attr.is_ignored(variant))
            .for_each(|variant| {
                let variant_name = &variant.ident;
                let index = variant_index_map[variant_name];
                let display_variant_name = variant_name.to_string();

                let check_fn_name = format_ident!("check_{}", display_variant_name.to_lowercase());
                check_quotes.push(quote! {
                    /// Check the variant's frequency.
                    #[cfg(feature = "check")]
                    #[inline]
                    #vis const fn #check_fn_name(&self) -> usize {
                        self.frequency[#index]
                    }
                });
                weighted_check_quotes.push(quote! {
                    /// Check the variant's weighted frequency.
                    #[cfg(feature = "check")]
                    #[inline]
                    #vis const fn #check_fn_name(&self) -> usize {
                        self.frequency[#index] * self.weight[#index]
                    }
                });
                aggregate_quotes.push(quote! {
                    (#display_variant_name, self.frequency[#index])
                });
                weighted_aggregate_quotes.push(quote! {
                    (#display_variant_name, self.frequency[#index] * self.weight[#index])
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

                weights.push(parsed_attr.weight.get(variant_name).copied().unwrap_or(1));

                let erase_fn_name = format_ident!("erase_{}", display_variant_name.to_lowercase());
                erase_quotes.push(quote! {
                    /// Erase a record.
                    /// It has no effect if you erase an ignored variant.
                    #[cfg(feature = "erase")]
                    #[inline]
                    #vis fn #erase_fn_name(&mut self) {
                        self.frequency[#index] = self.frequency[#index].saturating_sub(1);
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
            check_quotes,
            weighted_check_quotes,
            erase_quotes,
            aggregate_quotes,
            group_aggregate_quotes: parsed_attr
                .groups
                .iter()
                .map(|(group_name, idents)| {
                    let variant_quotes = idents
                        .iter()
                        .filter_map(|ident| variant_index_map.get(ident))
                        .map(|index| quote! { self.frequency[#index] })
                        .collect::<Vec<proc_macro2::TokenStream>>();
                    quote! {
                        (#group_name, #(#variant_quotes)+*)
                    }
                })
                .collect(),
            weighted_aggregate_quotes,
            weighted_group_aggregate_quotes: parsed_attr
                .groups
                .iter()
                .map(|(group_name, idents)| {
                    let variant_quotes = idents
                        .iter()
                        .filter_map(|ident| variant_index_map.get(ident))
                        .map(|index| quote! { self.frequency[#index] * self.weight[#index] })
                        .collect::<Vec<proc_macro2::TokenStream>>();
                    quote! {
                        (#group_name, #(#variant_quotes)+*)
                    }
                })
                .collect(),
            has_customized_group: parsed_attr.has_customized_group,
        }
    }
}
