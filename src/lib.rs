extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Path, Type, TypePath};

#[proc_macro_derive(Deref, attributes(deref_target))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();
    let (field_ty, field_access) = parse_fields(&item, false);

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote!(
        impl #impl_generics core::ops::Deref for #name #ty_generics
        #where_clause
        {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                #field_access
            }
        }
    ).into()
}

#[proc_macro_derive(DerefMut, attributes(deref_target))]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();
    let (_, field_access) = parse_fields(&item, true);

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote!(
        impl #impl_generics core::ops::DerefMut for #name #ty_generics
        #where_clause
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                #field_access
            }
        }
    ).into()
}

fn parse_fields(item: &syn::DeriveInput, mutable: bool) -> (syn::Type, proc_macro2::TokenStream) {
    let trait_name = if mutable { "DerefMut" } else { "Deref" };
    let fields = match item.data {
        syn::Data::Struct(ref body) =>
            body.fields.iter()
                .filter(|field| {
                    if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = &field.ty {
                        if segments
                            .last()
                            .expect("Expected path to have at least one segment")
                            .ident == "PhantomData"
                        {
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                })
                .collect::<Vec<&syn::Field>>(),
        _ => panic!("#[derive({})] can only be used on structs", trait_name),
    };

    let target_field = match fields.len() {
        1 => fields[0],
        _ => {
            // Look for attribute
            let mut targets = Vec::new();
            for field in fields.iter() {
                if let Some(_) = field.attrs.iter().find(|attr| attr.path.is_ident("deref_target")) {
                    targets.push(field);
                }
            }
            match targets.len() {
                0 => panic!("#[derive({})]: since there is more than one field, #[deref_target] is needed on a field", trait_name),
                1 => targets[0],
                _ => panic!("#[derive({})]: only one #[deref_target] allowed", trait_name),
            }

        }
    };



    let field_name = match target_field.ident {
        Some(ref ident) => quote!(#ident),
        None => quote!(0),
    };

    match (target_field.ty.clone(), mutable) {
        (syn::Type::Reference(syn::TypeReference { elem, .. }), _) => (*elem, quote!(self.#field_name)),
        (x, true) => (x, quote!(&mut self.#field_name)),
        (x, false) => (x, quote!(&self.#field_name)),
    }
}
