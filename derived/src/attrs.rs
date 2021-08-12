use proc_macro2::Ident;
use syn::{DataEnum, Variant};

#[derive(Debug)]
pub struct ParsedAttr {
    ignored: Vec<Ident>,
}

impl ParsedAttr {
    pub fn parse(data_enum: &DataEnum) -> ParsedAttr {
        let mut parsed = ParsedAttr { ignored: vec![] };
        data_enum.variants.iter().enumerate().for_each(|item| {
            parsed.parse_variant_attributes(item);
        });
        parsed
    }

    fn parse_variant_attributes(&mut self, (_index, variant): (usize, &Variant)) {
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
                                self.ignored.push(variant.ident.clone());
                            }
                            syn::Meta::List(_) => {}
                            _ => {}
                        },
                        _ => {}
                    });
                }
                _ => {}
            });
    }

    pub fn is_ignored(&self, variant: &Variant) -> bool {
        self.ignored.contains(&variant.ident)
    }

    pub fn ignored_count(&self) -> usize {
        self.ignored.len()
    }
}
