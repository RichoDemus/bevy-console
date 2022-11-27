use better_bae::{FromAttributes, TryFromAttributes};
use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[derive(Debug, Eq, PartialEq, FromAttributes)]
#[bae("console_command")]
struct ConsoleCommandContainerAttr {
    name: syn::Lit,
}

/// Implement
/// [`CommandName`](https://docs.rs/bevy_console/latest/bevy_console/trait.CommandName.html),
/// [`CommandArgs`](https://docs.rs/bevy_console/latest/bevy_console/trait.CommandArgs.html) and
/// [`CommandHelp`](https://docs.rs/bevy_console/latest/bevy_console/trait.CommandHelp.html)
/// for a struct.
///
/// Doc comments are used to provide argument and command help.
///
/// # Container Attributes
///
/// - `#[console_command(name = "log")`
///
///   Specify the console command name.
///
/// # Example
///
/// ```ignore
/// /// Prints given arguments to the console.
/// #[derive(ConsoleCommand)]
/// #[console_command(name = "log")]
/// struct LogCommand {
///     /// Message to print
///     msg: String,
///     /// Number of times to print message
///     num: Option<i64>,
/// }
/// ```
#[proc_macro_derive(ConsoleCommand, attributes(console_command))]
pub fn derive_console_command(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = &ast.ident;

    let named_fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields.named,
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => syn::punctuated::Punctuated::default(),
        _ => {
            return syn::Error::new(
                Span::call_site().into(),
                "only structs with named fields and unit structs are supported",
            )
            .into_compile_error()
            .into();
        }
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
                .into();
        }
    };

    let mut fields = Vec::with_capacity(named_fields.len());
    let mut previous_optional = None;
    for (i, syn::Field { ident, ty, .. }) in named_fields.iter().enumerate() {
        let optional = is_ty_option(ty);
        if !optional {
            if let Some(previous_optional) = previous_optional {
                return TokenStream::from_iter([
                    TokenStream::from(
                        syn::Error::new_spanned(
                            ty,
                            "field is required, but an optional field is defined above this field - all optional fields must be placed last",
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

        let index = i as u8;

        let expanded = quote! {
            #ident: <#ty as bevy_console::FromValue>::from_value_iter(&mut values, #index)?,
        };
        fields.push(expanded);
    }

    let doc_comments = get_doc_comments(&ast.attrs);
    let command_description = if !doc_comments.is_empty() {
        let description = doc_comments.join("\n");
        quote! {
            Some(#description.to_string())
        }
    } else {
        quote! {
            None
        }
    };
    let command_arg_info = named_fields.iter().map(
        |syn::Field {
             attrs, ident, ty, ..
         }| {
            let name = ident.as_ref().unwrap().to_string();
            let ty_string = ty_to_string(ty)
                .map(|ty_string| quote!(#ty_string))
                .unwrap_or_else(|| quote!(stringify!(#ty)));
            let doc_comments = get_doc_comments(attrs);
            let arg_description = if !doc_comments.is_empty() {
                let description = doc_comments.join("\n");
                quote! {
                    Some(#description.to_string())
                }
            } else {
                quote! {
                    None
                }
            };
            let optional = is_ty_option(ty);

            quote! {
                bevy_console::CommandArgInfo {
                    name: #name.to_string(),
                    ty: #ty_string.to_string(),
                    description: #arg_description,
                    optional: #optional,
                }
            }
        },
    );

    TokenStream::from(quote! {
        #[automatically_derived]
        impl bevy_console::CommandName for #ident {
            fn command_name() -> &'static str {
                #command_name
            }
        }

        #[automatically_derived]
        impl bevy_console::CommandArgs for #ident {
            fn from_values(values: &[bevy_console::ValueRawOwned]) -> ::std::result::Result<Self, bevy_console::FromValueError> {
                let mut values = values.iter();

                Ok(#ident {
                    #( #fields )*
                })
            }
        }

        #[automatically_derived]
        impl bevy_console::CommandHelp for #ident {
            fn command_help() -> ::std::option::Option<bevy_console::CommandInfo> {
                ::std::option::Option::Some(bevy_console::CommandInfo {
                    name: #command_name.to_string(),
                    description: #command_description,
                    args: vec![
                        #( #command_arg_info, )*
                    ],
                })
            }
        }

        #[automatically_derived]
        impl bevy::prelude::Resource for #ident {}
    })
}

fn get_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs.iter().fold(Vec::new(), |mut acc, attr| {
        match attr.parse_meta() {
            Ok(syn::Meta::NameValue(syn::MetaNameValue {
                path,
                lit: syn::Lit::Str(comment),
                ..
            })) if path
                .segments
                .first()
                .map(|segment| segment.ident == "doc")
                .unwrap_or(false) =>
            {
                acc.push(comment.value().trim().to_string())
            }
            _ => {}
        }

        acc
    })
}

fn is_ty_option(ty: &syn::Type) -> bool {
    let mut ty_string = ty.to_token_stream().to_string();
    ty_string.retain(|c| c != ' ');

    ty_string.starts_with("Option<")
        || ty_string.starts_with("option::Option<")
        || ty_string.starts_with("std::option::Option<")
        || ty_string.starts_with("::std::option::Option<")
}

fn ty_to_string(ty: &syn::Type) -> Option<&'static str> {
    let mut ty_string = ty.to_token_stream().to_string();
    ty_string.retain(|c| c != ' ');

    let optional = is_ty_option(ty);

    let inner_ty = if optional {
        ty_string
            .trim_start_matches("::std::option::Option")
            .trim_start_matches("std::option::Option")
            .trim_start_matches("option::Option")
            .trim_start_matches("Option")
            .trim_start_matches('<')
            .trim_end_matches('>')
    } else {
        &ty_string
    };

    match inner_ty {
        "String" | "string::String" | "std::string::String" | "::std::string::String" => {
            Some("string")
        }
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => Some("int"),
        "f32" | "f64" => Some("float"),
        "bool" => Some("bool"),
        _ => None,
    }
}
