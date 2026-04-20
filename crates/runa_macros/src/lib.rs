use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RunaComponent)]
pub fn derive_runa_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    TokenStream::from(quote! {
        impl ::runa_engine::runa_core::components::Component for #ident {
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any {
                self
            }
        }

        impl ::runa_engine::RunaComponentType for #ident {
            fn runa_component_type_name() -> &'static str {
                concat!(module_path!(), "::", stringify!(#ident))
            }
        }

        impl #ident {
            pub fn register(engine: &mut ::runa_engine::Engine) -> ::runa_engine::TypeMetadata {
                engine.register_component_named::<Self>(
                    <Self as ::runa_engine::RunaComponentType>::runa_component_type_name()
                )
            }
        }

        impl ::runa_engine::RunaTypeRegistration for #ident {
            fn register(
                engine: &mut ::runa_engine::Engine
            ) -> ::runa_engine::TypeMetadata {
                #ident::register(engine)
            }
        }
    })
}

#[proc_macro_derive(RunaScript)]
pub fn derive_runa_script(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    TokenStream::from(quote! {
        impl ::runa_engine::RunaScriptType for #ident {
            fn runa_script_type_name() -> &'static str {
                concat!(module_path!(), "::", stringify!(#ident))
            }
        }

        impl #ident {
            pub fn register(engine: &mut ::runa_engine::Engine) -> ::runa_engine::TypeMetadata {
                engine.register_script_named::<Self>(
                    <Self as ::runa_engine::RunaScriptType>::runa_script_type_name()
                )
            }
        }

        impl ::runa_engine::RunaTypeRegistration for #ident {
            fn register(
                engine: &mut ::runa_engine::Engine
            ) -> ::runa_engine::TypeMetadata {
                #ident::register(engine)
            }
        }
    })
}

#[proc_macro_derive(RunaArchetype, attributes(runa))]
pub fn derive_runa_archetype(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();
    let archetype_name = archetype_name_override(&input)
        .unwrap_or_else(|| to_snake_case(&ident.to_string()));

    TokenStream::from(quote! {
        impl ::runa_engine::RunaArchetype for #ident {
            fn key() -> ::runa_engine::ArchetypeKey {
                ::runa_engine::ArchetypeKey::new(#archetype_name)
            }

            fn create(
                world: &mut ::runa_engine::runa_core::ocs::World
            ) -> ::runa_engine::runa_core::ocs::ObjectId {
                #ident::create(world)
            }
        }

        impl #ident {
            pub fn archetype_key() -> ::runa_engine::ArchetypeKey {
                <Self as ::runa_engine::RunaArchetype>::key()
            }

            pub fn archetype_name() -> &'static str {
                #archetype_name
            }

            pub fn register(
                engine: &mut ::runa_engine::Engine
            ) -> ::runa_engine::ArchetypeMetadata {
                engine.register_archetype::<Self>()
            }
        }
    })
}

fn archetype_name_override(input: &DeriveInput) -> Option<String> {
    for attribute in &input.attrs {
        if !attribute.path().is_ident("runa") {
            continue;
        }

        let mut value = None;
        let _ = attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                let literal: syn::LitStr = meta.value()?.parse()?;
                value = Some(literal.value());
            }
            Ok(())
        });

        if value.is_some() {
            return value;
        }
    }

    None
}

fn to_snake_case(value: &str) -> String {
    let mut result = String::new();
    let mut previous_was_lowercase_or_digit = false;

    for ch in value.chars() {
        if ch.is_ascii_uppercase() {
            if previous_was_lowercase_or_digit && !result.ends_with('_') {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            previous_was_lowercase_or_digit = false;
        } else if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
            previous_was_lowercase_or_digit = true;
        } else if !result.ends_with('_') && !result.is_empty() {
            result.push('_');
            previous_was_lowercase_or_digit = false;
        }
    }

    result.trim_matches('_').to_string()
}
