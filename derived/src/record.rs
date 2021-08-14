use quote::quote;
use syn::DeriveInput;

pub(crate) fn generate_record_fn(
    input: &DeriveInput,
    match_arm_quotes: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let (_, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        #vis fn record#ty_generics(&mut self, target: &#name#ty_generics) {
            let index = match target {
                #(#match_arm_quotes,)*
                _ => None,
            };

            if let Some(index) = index {
                self.container[index] = self.container[index].saturating_add(1);
            }
        }
    }
}

pub(crate) fn generate_weight_record_fn(
    input: &DeriveInput,
    weight_match_arm_quotes: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let (_, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        #vis fn record#ty_generics(&mut self, target: &#name#ty_generics) {
            let pair = match target {
                #(#weight_match_arm_quotes,)*
                _ => None,
            };

            if let Some((index, weight)) = pair {
                self.container[index] = self.container[index].saturating_add(weight);
            }
        }
    }
}
