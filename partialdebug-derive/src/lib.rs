use proc_macro::TokenStream;

use quote::quote;
use syn::*;

#[proc_macro_derive(NonExhaustivePartialDebug)]
pub fn derive_non_exhaustive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.fields {
        Fields::Named(FieldsNamed { named, .. }) => named,
        Fields::Unnamed(_) => unimplemented!(),
        Fields::Unit => unimplemented!(),
    };

    let as_debug_all_fields = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            match ::partialdebug::AsDebug::as_debug(&self. #name) {
                None => {
                    exhaustive = false;
                }
                Some(field) => {
                    s.field(stringify!(#name), field);
                }
            }
        }
    });

    let expanded = quote! {
        impl #impl_generics ::core::fmt::Debug for #name #ty_generics #where_clause{
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut s = f.debug_struct(stringify!(#name));
                let mut exhaustive = false;

                #(#as_debug_all_fields)*

                if exhaustive {
                    s.finish()
                } else {
                    s.finish_non_exhaustive()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
