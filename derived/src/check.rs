use quote::quote;
use syn::DeriveInput;

#[cfg(feature = "check")]
pub(crate) fn generate_check_fn(
    input: &DeriveInput,
    match_arm_quotes: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let (_, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        #vis const fn check#ty_generics(&self, target: &#name#ty_generics) -> Option<usize> {
            let index = match target {
                #(#match_arm_quotes,)*
                _ => None,
            };

            if let Some(index) = index {
                Some(self.container[index])
            } else {
                None
            }
        }
    }
}

#[cfg(not(feature = "check"))]
pub(crate) fn generate_check_fn(
    _input: &DeriveInput,
    _match_arm_quotes: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {}
}
