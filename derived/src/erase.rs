use quote::quote;
use syn::DeriveInput;

#[cfg(feature = "erase")]
pub(crate) fn generate_erase_fn(
    input: &DeriveInput,
    match_arm_quotes: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let (_, ty_generics, _) = input.generics.split_for_impl();

    quote! {
        #vis fn erase#ty_generics(&mut self, target: &#name#ty_generics) {
            let index = match target {
                #(#match_arm_quotes,)*
                _ => None,
            };

            if let Some(index) = index {
                self.container[index] = self.container[index].saturating_sub(1);
            }
        }
    }
}

#[cfg(not(feature = "erase"))]
pub(crate) fn generate_erase_fn(
    _input: &DeriveInput,
    _match_arm_quotes: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    quote! {}
}
