use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
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
                    __exhaustive = false;
                }
                Some(field) => {
                    __s.field(stringify!(#name), field);
                }
            }
        }
    });

    let expanded = quote! {
        impl #impl_generics ::core::fmt::Debug for #name #ty_generics #where_clause{
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut __s = f.debug_struct(stringify!(#name));
                let mut __exhaustive = false;

                #(#as_debug_all_fields)*

                if __exhaustive {
                    __s.finish()
                } else {
                    __s.finish_non_exhaustive()
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// The placeholder version of `PartialDebug`
#[proc_macro_derive(PlaceholderPartialDebug, attributes(debug_placeholder))]
pub fn derive_placeholder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let placeholder = match get_placeholder(&input) {
        Ok(placeholder) => placeholder,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let implementation = match input.data {
        Data::Struct(DataStruct { fields, .. }) => gen_variant_debug(
            &fields,
            &name,
            struct_field_conversions(&fields, &placeholder),
        ),
        Data::Enum(data_enum) => gen_enum_debug(&data_enum, &name, &placeholder),
        Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics ::core::fmt::Debug for #name #ty_generics #where_clause{
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #implementation
            }
        }
    };

    TokenStream::from(expanded)
}

fn gen_variant_debug(
    fields: &Fields,
    variant_name: &Ident,
    field_conversions: impl Iterator<Item = TokenStream2>,
) -> TokenStream2 {
    let constructor = match fields {
        Fields::Named(_) => quote! {debug_struct},
        Fields::Unnamed(_) | Fields::Unit => quote! {debug_tuple},
    };

    quote! {
        f.#constructor(stringify!(#variant_name))
        #(#field_conversions)*
        .finish()
    }
}

fn gen_enum_debug(
    data_enum: &DataEnum,
    enum_name: &Ident,
    placeholder: &Option<String>,
) -> TokenStream2 {
    let all_variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let match_content = gen_variant_debug(
            &variant.fields,
            variant_name,
            enum_field_conversions(&variant.fields, placeholder),
        );
        let match_pattern = gen_match_pattern(enum_name, variant);
        quote! {
            #match_pattern => {
                #match_content
            }
        }
    });

    quote! {
        match self {
            #(#all_variants)*
        }
    }
}

fn struct_field_conversions<'a>(
    fields: &'a Fields,
    placeholder: &'a Option<String>,
) -> impl Iterator<Item = TokenStream2> + 'a {
    fields.iter().enumerate().map(move |(idx, field)| {
        let (field_handle, name_arg) = match &field.ident {
            None => {
                let index = Index::from(idx);
                (quote! {self.#index}, None)
            }
            Some(name) => (quote! {self.#name}, Some(quote! {stringify!(#name),})),
        };
        gen_field_as_debug(field, placeholder, field_handle, name_arg)
    })
}

fn enum_field_conversions<'a>(
    fields: &'a Fields,
    placeholder: &'a Option<String>,
) -> impl Iterator<Item = TokenStream2> + 'a {
    fields.iter().enumerate().map(move |(idx, field)| {
        let (field_handle, name_arg) = match &field.ident {
            None => {
                let ident = Ident::new(&format!("__{}", idx), field.span());
                (quote! {#ident}, None)
            }
            Some(name) => (quote! {#name}, Some(quote! {stringify!(#name),})),
        };
        gen_field_as_debug(field, placeholder, field_handle, name_arg)
    })
}

fn gen_field_as_debug(
    field: &Field,
    placeholder: &Option<String>,
    field_handle: TokenStream2,
    name_arg: Option<TokenStream2>,
) -> TokenStream2 {
    let type_name = get_type_name(&field.ty);

    // type name or given placeholder string
    let placeholder_string = placeholder.as_ref().unwrap_or(&type_name);

    quote! {
        .field(
            #name_arg
            match ::partialdebug::AsDebug::as_debug(&#field_handle){
                None => &::partialdebug::Placeholder(#placeholder_string),
                Some(__field) => __field,
            },
        )
    }
}

fn gen_match_pattern(enum_name: &Ident, variant: &Variant) -> TokenStream2 {
    let variant_name = &variant.ident;
    let destructuring_pattern = match &variant.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let patterns = named.iter().map(|field| &field.ident);
            quote! {
                {#(#patterns),*}
            }
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let patterns = unnamed
                .iter()
                .enumerate()
                .map(|(idx, field)| Ident::new(&format!("__{}", idx), field.span()));
            quote! {
                (#(#patterns),*)
            }
        }
        Fields::Unit => TokenStream2::new(),
    };

    quote! {#enum_name::#variant_name #destructuring_pattern}
}

struct Placeholder(String);

impl Parse for Placeholder {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![=]>()?;
        Ok(Placeholder(input.parse::<LitStr>()?.value()))
    }
}

/// Tries to parse a placeholder string if there is one
fn get_placeholder(input: &DeriveInput) -> Result<Option<String>> {
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

/// returns the type as a string with unnecessary whitespace removed
fn get_type_name(ty: &Type) -> String {
    let mut type_name = String::new();
    let chars: Vec<char> = ty.to_token_stream().to_string().trim().chars().collect();

    for (i, char) in chars.iter().enumerate() {
        if char.is_whitespace() {
            // remove whitespace surrounding punctuation
            // exceptions are:
            //      - whitespace surrounding `->`
            //      - whitespace following `,` or `;`
            let (before, after) = (chars[i - 1], chars[i + 1]); // always valid because string was trimmed before
            let before_wide = chars.get(i.saturating_sub(2)..i);
            let after_wide = chars.get(i + 1..=i + 2);

            if (before.is_ascii_punctuation() || after.is_ascii_punctuation())
                && !matches!(before, ';' | ',')
                && !matches!(before_wide, Some(['-', '>']))
                && !matches!(after_wide, Some(['-', '>']))
            {
                continue;
            }
        }

        type_name.push(*char);
    }

    type_name
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_type_name_formatting(type_str: &str) {
        let ty: Type = parse_str(type_str).unwrap();
        assert_eq!(get_type_name(&ty), type_str)
    }

    #[test]
    fn test_no_spaces() {
        test_type_name_formatting("u8");
        test_type_name_formatting("Option<u8>");
        test_type_name_formatting("[u8]");
        test_type_name_formatting("()");
        test_type_name_formatting("std::fmt::Formatter<'_>");
    }
    #[test]
    fn test_array() {
        test_type_name_formatting("[u8; 4]");
    }
    #[test]
    fn test_lifetime() {
        test_type_name_formatting("&'a u8");
    }
    #[test]
    fn test_function() {
        test_type_name_formatting("fn(u8) -> u8");
    }
    #[test]
    fn test_trait_object() {
        test_type_name_formatting("Box<dyn Send>");
    }
    #[test]
    fn test_tuple() {
        test_type_name_formatting("(Option<u8>, u8)");
    }
}
