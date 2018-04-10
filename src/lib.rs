extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Deref)]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();
    let (field_ty, field_access) = parse_fields(&item, false);

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote!(
        impl #impl_generics ::std::ops::Deref for #name #ty_generics
        #where_clause
        {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                #field_access
            }
        }
    ).into()
}

#[proc_macro_derive(DerefMut)]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();
    let (_, field_access) = parse_fields(&item, true);

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote!(
        impl #impl_generics ::std::ops::DerefMut for #name #ty_generics
        #where_clause
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                #field_access
            }
        }
    ).into()
}

fn parse_fields(item: &syn::DeriveInput, mutable: bool) -> (syn::Type, quote::Tokens) {
    let trait_name = if mutable {
        "DerefMut"
    } else {
        "Deref"
    };
    let fields = match item.data {
        syn::Data::Struct(ref body) => body.fields.iter().collect::<Vec<&syn::Field>>(),
        _ => panic!("#[derive({})] can only be used on structs", trait_name),
    };

    let field_ty = if fields.len() == 1 {
        fields[0].ty.clone()
    } else {
        panic!("#[derive({})] can only be used on structs with one field", trait_name)
    };
    let field_name = match fields[0].ident {
        Some(ident) => quote!(#ident),
        None => quote!(0)
    };

    match (field_ty, mutable) {
        (syn::Type::Reference(syn::TypeReference { elem, .. }), _) => (*elem.clone(), quote!(self.#field_name)),
        (x, true) => (x, quote!(&mut self.#field_name)),
        (x, false) => (x, quote!(&self.#field_name)),
    }
}
