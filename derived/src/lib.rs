extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(VariantCount)]
pub fn derive_variant_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let counter_struct = format_ident!("{}Counter", name);
    let expanded = quote! {
        #[must_use]
        pub struct #counter_struct {
            container: &'static [usize],
        }

        impl #counter_struct {
            pub fn new() -> #counter_struct {
                let container: &[usize; 3] = &[0, 0, 0];
                #counter_struct { container }
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
    // expanded.parse().unwrap()
}
