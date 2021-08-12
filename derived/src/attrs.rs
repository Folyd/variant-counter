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
                if variant.attrs.is_empty() {
                    parsed.record_group(variant.ident.to_string(), variant.ident.clone());
                } else {
                    parsed.parse_variant_attributes(variant);
                }
            });
        parsed
    }

    fn parse_variant_attributes(&mut self, variant: &Variant) {
        variant
            .attrs
            .iter()
            .for_each(|attr| match attr.parse_meta() {
                Ok(syn::Meta::List(meta_list))
                    if meta_list
                        .path
                        .get_ident()
                        .filter(|&ident| ident == "counter")
                        .is_some() =>
                {
                    meta_list.nested.iter().for_each(|nested| match nested {
                        syn::NestedMeta::Meta(meta) => match meta {
                            syn::Meta::Path(path)
                                if path
                                    .get_ident()
                                    .filter(|&ident| ident == "ignore")
                                    .is_some() =>
                            {
                                self.ignores.push(variant.ident.clone());
                            }
                            syn::Meta::NameValue(name_value) => {
                                match name_value.path.get_ident().map(|ident| ident.to_string()) {
                                    Some(name) if name == "group" => {
                                        match &name_value.lit {
                                            syn::Lit::Str(str) => self
                                                .record_group(str.value(), variant.ident.clone()),
                                            _ => panic!(
                                                "Invalid group value type: #[counter(group = `string type` )]",
                                            ),
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    });
                }
                _ => {}
            });
    }

    fn record_group(&mut self, name: String, ident: Ident) {
        self.groups.entry(name).or_insert_with(Vec::new).push(ident);
    }

    pub(crate) fn index_group(&self, variant: &Variant) -> Option<(usize, &String)> {
        self.groups
            .iter()
            .enumerate()
            .find_map(|(index, (name, ident))| {
                if ident.contains(&variant.ident) {
                    Some((index, name))
                } else {
                    None
                }
            })
    }

    pub(crate) fn is_ignored(&self, variant: &Variant) -> bool {
        self.ignores.contains(&variant.ident)
    }
}
