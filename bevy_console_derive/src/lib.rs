use better_bae::{FromAttributes, TryFromAttributes};
use bevy_macro_utils::get_named_struct_fields;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[derive(Debug, Eq, PartialEq, FromAttributes)]
#[bae("console_command")]
struct ConsoleCommandContainerAttr {
    name: syn::Lit,
}

#[proc_macro_derive(ConsoleCommand, attributes(console_command))]
pub fn derive_console_command(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = &ast.ident;

    let named_fields = match get_named_struct_fields(&ast.data) {
        Ok(fields) => &fields.named,
        Err(err) => return err.into_compile_error().into(),
    };

    let attrs = match ConsoleCommandContainerAttr::from_attributes(&ast.attrs) {
        Ok(attrs) => attrs,
        Err(err) => return err.into_compile_error().into(),
    };

    let command_name = match attrs.name {
        syn::Lit::Str(name) => name.value(),
        _ => {
            return syn::Error::new_spanned(attrs.name, "name must be a string literal")
                .into_compile_error()
                .into()
        }
    };

    let mut fields = Vec::with_capacity(named_fields.len());
    let mut previous_optional = None;
    for (i, syn::Field { ident, ty, .. }) in named_fields.iter().enumerate() {
        let mut ty_string = ty.to_token_stream().to_string();
        ty_string.retain(|c| c != ' ');

        let optional = ty_string.starts_with("Option<")
            || ty_string.starts_with("option::Option<")
            || ty_string.starts_with("std::option::Option<")
            || ty_string.starts_with("::std::option::Option<");
        if !optional {
            if let Some(previous_optional) = previous_optional {
                return TokenStream::from_iter([
                    TokenStream::from(
                        syn::Error::new_spanned(
                            ty,
                            "field is required, but an optional field is defined above this field - all optional fields must be placed last"
                        )
                            .into_compile_error()
                    ),
                    TokenStream::from(
                        syn::Error::new(previous_optional, "all optional fields must be after required fields")
                            .into_compile_error()
                    ),
                ]);
            }
        }
        if optional && previous_optional.is_none() {
            previous_optional = Some(ty.span());
        }

        // let inner_ty = if optional {
        //     ty_string
        //         .trim_start_matches("::std::option::Option")
        //         .trim_start_matches("std::option::Option")
        //         .trim_start_matches("option::Option")
        //         .trim_start_matches("Option")
        //         .trim_start_matches("<")
        //         .trim_end_matches(">")
        //         .to_token_stream()
        // } else {
        //     field.ty.to_token_stream()
        // };
        let index = i as u8;

        let expanded = quote! {
            #ident: <#ty as ::bevy_console::FromValue>::from_value_iter(&mut values, #index)?,
        };
        fields.push(expanded);
    }

    TokenStream::from(quote! {
        impl ::bevy_console::CommandName for #ident {
            fn command_name() -> &'static str {
                #command_name
            }
        }

        impl ::bevy_console::CommandArgs for #ident {
            fn from_values(values: &[::bevy_console::ValueRawOwned]) -> ::std::result::Result<Self, ::bevy_console::FromValueError> {
                let mut values = values.iter();

                Ok(#ident {
                    #( #fields )*
                })
            }
        }
    })
}
