use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ConsoleCommand, attributes(command))]
pub fn derive_clap_command(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let name_string = get_command_name(&derive_input);
    let name = &derive_input.ident;
    let generics = derive_input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics bevy_console::NamedCommand for #name #ty_generics #where_clause {
            fn name() -> &'static str {
                #name_string
            }
        }

        impl #impl_generics bevy::prelude::Resource for #name #ty_generics #where_clause {};
    })
}

fn get_command_name(input: &DeriveInput) -> syn::LitStr {
    input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path.is_ident("command") {
                if let Ok(syn::Meta::List(list)) = attr.parse_meta() {
                    return list.nested.iter().find_map(|meta| {
                        if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = meta {
                            Some(nv.lit.clone())
                        } else {
                            None
                        }
                    });
                }
            }
            None
        })
        .map(|lit| {
            if let syn::Lit::Str(str) = lit {
                str
            } else {
                panic!("Expected string literal as command name");
            }
        })
        .unwrap_or_else(|| syn::LitStr::new(&input.ident.to_string(), input.ident.span()))
}
