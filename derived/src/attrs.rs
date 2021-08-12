use std::collections::BTreeMap;

use proc_macro2::Ident;
use syn::{DataEnum, Variant};

#[derive(Debug)]
pub(crate) struct ParsedAttr {
    pub(crate) ignores: Vec<Ident>,
    pub(crate) groups: BTreeMap<String, Vec<Ident>>,
}

impl ParsedAttr {
    pub fn parse(data_enum: &DataEnum) -> ParsedAttr {
        let mut parsed = ParsedAttr {
            ignores: vec![],
            groups: BTreeMap::default(),
        };
        data_enum
            .variants
            .iter()
            .enumerate()
            .for_each(|(_index, variant)| {
                if !parsed.parse_variant_attributes(variant) {
                    // If not desired attributes has been parsed,
                    // record the variant as the group as normal.
                    parsed.record_group(variant.ident.to_string(), variant.ident.clone());
                }
            });
        parsed.validate_legality();
        parsed
    }

    // Reture true if desired attributes has been parsed, false otherwise.
    fn parse_variant_attributes(&mut self, variant: &Variant) -> bool {
        variant.attrs.iter().any(|attr| match attr.parse_meta() {
            Ok(syn::Meta::List(meta_list))
                if meta_list
                    .path
                    .get_ident()
                    .filter(|&ident| ident == "counter")
                    .is_some() =>
            {
                meta_list.nested.iter().any(|nested| match nested {
                    syn::NestedMeta::Meta(syn::Meta::Path(path)) => match path.get_ident() {
                        Some(ident) if ident == "ignore" => {
                            self.ignores.push(variant.ident.clone());
                            true
                        }
                        Some(ident) => panic!("Invalid attribute: {}", ident.to_string()),
                        _ => false,
                    },
                    syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                        match name_value.path.get_ident().map(|ident| ident.to_string()) {
                            Some(name) if name == "group" => match &name_value.lit {
                                syn::Lit::Str(str) => {
                                    self.record_group(str.value(), variant.ident.clone());
                                    true
                                }
                                _ => panic!(
                                    "Invalid group value type: #[counter(group = `string type` )]",
                                ),
                            },
                            Some(invalid_name) => {
                                panic!("Invalid attribute: {}", invalid_name);
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                })
            }
            _ => false,
        })
    }

    fn record_group(&mut self, name: String, ident: Ident) {
        self.groups.entry(name).or_insert_with(Vec::new).push(ident);
    }

    pub(crate) fn index_group(&self, variant: &Ident) -> Option<(usize, &String)> {
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
