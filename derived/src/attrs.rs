use std::collections::{BTreeMap, HashMap};

use quote::quote_spanned;
use syn::{DataEnum, Variant};

#[derive(Debug)]
pub(crate) struct ParsedAttr {
    pub(crate) ignores: Vec<proc_macro2::Ident>,
    pub(crate) groups: BTreeMap<String, Vec<proc_macro2::Ident>>,
    pub(crate) weight: HashMap<proc_macro2::Ident, usize>,
}

impl ParsedAttr {
    pub fn parse(data_enum: &DataEnum) -> Result<ParsedAttr, proc_macro2::TokenStream> {
        let mut parsed = ParsedAttr {
            ignores: vec![],
            groups: BTreeMap::default(),
            weight: HashMap::default(),
        };

        if data_enum.variants.is_empty() {
            panic!("Empty enum is not supported.");
        }

        for variant in data_enum.variants.iter() {
            parsed.parse_variant_attributes(variant)?;
        }

        if parsed.ignores.len() == data_enum.variants.len() {
            panic!("All variants were ignored, please check again.");
        }

        parsed.validate_legality();
        Ok(parsed)
    }

    fn parse_variant_attributes(
        &mut self,
        variant: &Variant,
    ) -> Result<(), proc_macro2::TokenStream> {
        for attr in variant.attrs.iter() {
            match attr.parse_meta() {
                Ok(syn::Meta::List(meta_list))
                    if meta_list
                        .path
                        .get_ident()
                        .filter(|&ident| ident == "counter")
                        .is_some() =>
                {
                    for nested in meta_list.nested.iter() {
                        match nested {
                            syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                                match path.get_ident() {
                                    Some(ident) if ident == "ignore" => {
                                        self.ignores.push(variant.ident.clone());
                                    }
                                    Some(ident) => {
                                        return Err(quote_spanned! {ident.span()=>
                                            compile_error!("Unknown attribute");
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                                let ident = name_value.path.get_ident();
                                match ident {
                                    Some(name) if name == "group" => {
                                        if let syn::Lit::Str(str) = &name_value.lit {
                                            self.record_group(str.value(), variant.ident.clone());
                                        } else {
                                            return Err(quote_spanned! {name.span()=>
                                                compile_error!("Invalid `group` value type: expected string type: #[counter(group = `string type`)]");
                                            });
                                        }
                                    }
                                    Some(name) if name == "weight" => {
                                        if let syn::Lit::Int(value) = &name_value.lit {
                                            self.record_weight(
                                                value
                                                    .base10_parse()
                                                    .expect("`weight` value parse failed"),
                                                variant.ident.clone(),
                                            );
                                        } else {
                                            return Err(quote_spanned! {name.span()=>
                                                compile_error!("Invalid `weight` value type, expected int type: #[counter(weight = `int type`)]");
                                            });
                                        }
                                    }
                                    Some(invalid_name) => {
                                        return Err(quote_spanned! {invalid_name.span()=>
                                            compile_error!("Unknown attribute.");
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if self.is_ignored(variant) {
            return Ok(());
        }

        if self.index_group(&variant.ident).is_none() {
            self.record_group(variant.ident.to_string(), variant.ident.clone());
        }

        Ok(())
    }

    fn record_group(&mut self, name: String, ident: proc_macro2::Ident) {
        self.groups.entry(name).or_insert_with(Vec::new).push(ident);
    }

    fn record_weight(&mut self, value: usize, ident: proc_macro2::Ident) {
        self.weight.insert(ident, value);
    }

    pub(crate) fn index_group(&self, variant: &proc_macro2::Ident) -> Option<(usize, &String)> {
        self.groups
            .iter()
            .enumerate()
            .find_map(|(index, (name, ident))| {
                if ident.contains(&variant) {
                    Some((index, name))
                } else {
                    None
                }
            })
    }

    pub(crate) fn is_ignored(&self, variant: &Variant) -> bool {
        self.ignores.contains(&variant.ident)
    }

    pub(crate) fn has_weight(&self) -> bool {
        !self.weight.is_empty()
    }

    fn validate_legality(&self) {
        let conflict_names: Vec<_> = self
            .ignores
            .iter()
            .filter(|ident| self.index_group(ident).is_some())
            .map(|ident| ident.to_string())
            .collect();
        if !conflict_names.is_empty() {
            panic!(
                "#[counter(ignore)] is exclusive to other attributes: error variant: {:?}",
                conflict_names
            );
        }
    }
}
