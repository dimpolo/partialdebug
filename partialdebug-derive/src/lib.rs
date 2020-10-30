use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::*;

/// The non exhaustive version of `PartialDebug`
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

/// The placeholder version of `PartialDebug`
#[proc_macro_derive(PlaceholderPartialDebug, attributes(debug_placeholder))]
pub fn derive_placeholder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let placeholder = match get_placeholder(&input) {
        Ok(placeholder) => placeholder,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.fields {
        Fields::Named(FieldsNamed { named, .. }) => named,
        Fields::Unnamed(_) => unimplemented!(),
        Fields::Unit => unimplemented!(),
    };

    let as_debug_all_fields = fields.iter().map(|field| {
        let name = &field.ident;
        let mut type_name = field.ty.to_token_stream().to_string();
        type_name.retain(|c| !c.is_whitespace()); // remove whitespace

        // type name or given placeholder string
        let placeholder_string = placeholder.as_ref().unwrap_or(&type_name);

        quote! {
            .field(
                stringify!(#name),
                match ::partialdebug::AsDebug::as_debug(&self.#name){
                    None => &::partialdebug::Placeholder(#placeholder_string),
                    Some(field) => field,
                },
            )
        }
    });

    let expanded = quote! {
        impl #impl_generics ::core::fmt::Debug for #name #ty_generics #where_clause{
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#name))

                #(#as_debug_all_fields)*

                .finish()
            }
        }
    };

    TokenStream::from(expanded)
}

struct Placeholder(String);

impl Parse for Placeholder {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![=]>()?;
        Ok(Placeholder(input.parse::<LitStr>()?.value()))
    }
}

/// Tries to parse a placeholder string if there is one
fn get_placeholder(input: &ItemStruct) -> Result<Option<String>> {
    let placeholders: Vec<_> = input
        .attrs
        .iter()
        .filter(|attribute| attribute.path.is_ident("debug_placeholder"))
        .collect();

    if placeholders.len() > 1 {
        return Err(Error::new_spanned(
            placeholders[1],
            "More than one debug_placeholder attribute",
        ));
    }

    placeholders
        .first()
        .map(|attribute| {
            parse2::<Placeholder>(attribute.tokens.clone()).map(|placeholder| placeholder.0)
        })
        .transpose()
}
