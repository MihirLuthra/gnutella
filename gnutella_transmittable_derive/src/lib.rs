extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};

use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Error, Field, Fields,
    GenericParam, Generics, Ident, Index,
};

use quote::{format_ident, quote, quote_spanned};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span2::call_site(), $string)
            .to_compile_error()
            .into();
    };
}

/// This derive macro generates implementation for
/// [Serializable](gnutella::transmittable::Serializable),
/// [Deserializable](gnutella::transmittable::Deserializable)
/// and [Transmittable](gnutella::transmittable::Transmittable) trait.
///
/// If the fields of struct don't implement the desired trait,
/// an error will be raised when calling `<Type as Trait>::`.
///
/// If the struct contains generic types, the trait is implemented
/// such for a generic type `T`, `impl <T: Transmittable>`.

#[proc_macro_derive(Transmittable)]
pub fn derive_transmittable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ref name = input.ident;

    let parse_struct_res = match parse_struct(&input) {
        Ok(parse_struct_res) => parse_struct_res,
        Err(e) => return e,
    };

    // We take ref to the fields of struct because
    // they need to be used inside quote! macro.
    // Struct members can't be interpolated directly in quote!
    let ref serialize_funcs = parse_struct_res.serialize_funcs;
    let ref deserialize_funcs = parse_struct_res.deserialize_funcs;
    let ref struct_maker = parse_struct_res.struct_maker;

    let generics = add_trait_bound(input.generics, "Transmittable");

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Serializable for #name #ty_generics #where_clause {
            fn serialize_append(&self, mut v: std::vec::Vec<u8>)
                -> std::result::Result<std::vec::Vec<u8>, std::boxed::Box<dyn std::error::Error>>
            {
                #serialize_funcs
                Ok(v)
            }
        }

        impl #impl_generics Deserializable for #name #ty_generics #where_clause {
            fn deserialize(data: &[u8])
                -> std::result::Result<(Self, usize), std::boxed::Box<dyn std::error::Error>>
            {
                let mut start: usize = 0;
                #deserialize_funcs
                Ok((#struct_maker, start))
            }
        }

        impl #impl_generics Transmittable for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

/// This is the struct returned from [parse_struct].
struct ParseStructRes {
    /// `TokenStream2` containing code for all fields of struct
    /// to serialize them and append to the existing vector `v`.
    serialize_funcs: TokenStream2,

    /// `TokenStream2` containing code for all fields of struct
    /// to deserialize the bytes array into their corresponding type
    /// and increment `start` by `bytes_parsed` so that next field
    /// can know where to start deserializing in byte array.
    deserialize_funcs: TokenStream2,

    /// `TokenStream2` containing code for contruct an instance of struct.
    struct_maker: TokenStream2,
}

impl ParseStructRes {
    fn new() -> ParseStructRes {
        ParseStructRes {
            serialize_funcs: TokenStream2::new(),
            deserialize_funcs: TokenStream2::new(),
            struct_maker: TokenStream2::new(),
        }
    }
}

/// This function fills up an instance of [ParseStructRes]
/// while iterating over all fields of the struct.
///
/// The struct can be named, unnamed or unit.
/// In case of unit struct, basically nothing is done.
/// For named and unnamed structs, [gen_code_for_fields] is called
/// which fills up an instance of [ParseStructRes] over successive iterations.
fn parse_struct(derive_input: &DeriveInput) -> Result<ParseStructRes, TokenStream> {
    let mut parse_struct_res = ParseStructRes::new();

    let ref data = derive_input.data;
    let ref name = derive_input.ident;

    match data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields_named) => {
                for (field_no, field) in fields_named.named.iter().enumerate() {
                    gen_code_for_fields(&mut parse_struct_res, field, field_no);
                }

                // parse_struct_res.struct_maker can't be interpolated directly in
                // quote!(). So took a ref.
                let ref struct_maker = parse_struct_res.struct_maker;

                parse_struct_res.struct_maker = quote_spanned! {fields_named.span()=>
                    #name {
                        #struct_maker
                    }
                };
            }
            Fields::Unnamed(ref fields_unnamed) => {
                for (field_no, field) in fields_unnamed.unnamed.iter().enumerate() {
                    gen_code_for_fields(&mut parse_struct_res, field, field_no);
                }

                // parse_struct_res.struct_maker can't be interpolated directly in
                // quote!(). So took a ref.
                let ref struct_maker = parse_struct_res.struct_maker;

                parse_struct_res.struct_maker = quote_spanned! {fields_unnamed.span()=>
                    #name(#struct_maker)
                };
            }
            Fields::Unit => {
                parse_struct_res.struct_maker = quote! {
                    #name
                };
            }
        },
        _ => {
            return Err(derive_error!(
                "#[derive(Transmittable)] only works with struct"
            ))
        }
    };

    Ok(parse_struct_res)
}

/// This function is used by [parse_struct] to fill up an instance of [ParseStructRes]
/// upon successive iterations.
///
/// * `parse_struct_res` - Partially updated [ParseStructRes] instance.
/// * `field` - The field to work on.
/// * `field_no` - Position of the field. (useful for tuple structs, e.g., self.0, self.1 etc.)
fn gen_code_for_fields(parse_struct_res: &mut ParseStructRes, field: &Field, field_no: usize) {
    let ref field_type = field.ty;
    let ref field_name = field.ident;

    if let Some(field_name) = field_name {
        // Serialize the current field and update the vector `v`
        parse_struct_res
            .serialize_funcs
            .extend(quote_spanned! {field.span()=>
                let v = <#field_type as Serializable>::serialize_append(&self.#field_name, v)?;
            });

        // Deserialize bytes to generate an instance of current field's type
        // and update `start` by incrementing `bytes_parsed`.
        parse_struct_res
            .deserialize_funcs
            .extend(quote_spanned! {field.span()=>
                let (#field_name, bytes_parsed)
                    = <#field_type as Deserializable>::deserialize(&data[start..])?;
                start += bytes_parsed;
            });

        // This is used to make a new named struct instance.
        parse_struct_res
            .struct_maker
            .extend(quote_spanned! {field.span()=>
                #field_name,
            });
    } else {
        let tuple_index = Index::from(field_no);

        // Serialize the current field and update the vector `v`
        parse_struct_res
            .serialize_funcs
            .extend(quote_spanned! {field.span()=>
                let v = <#field_type as Serializable>::serialize_append(&self.#tuple_index, v)?;
            });

        let deserialized_ident = format_ident!("deserialized_{}", field_no);

        // Deserialize bytes to generate an instance of current field's type
        // and update `start` by incrementing `bytes_parsed`.
        parse_struct_res
            .deserialize_funcs
            .extend(quote_spanned! {field.span()=>
                let (#deserialized_ident, bytes_parsed)
                    = <#field_type as Deserializable>::deserialize(&data[start..])?;
                start += bytes_parsed;
            });

        // This is used to make a new tuple struct instance.
        parse_struct_res
            .struct_maker
            .extend(quote_spanned! {field.span()=>
                #deserialized_ident,
            });
    }
}

/// This function adds the a trait bound specified by `bound` to
/// all the type params in `generics`.
fn add_trait_bound(mut generics: Generics, bound: &str) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = param {
            let bound_ident = Ident::new(bound, type_param.span());
            type_param.bounds.push(parse_quote!(#bound_ident));
        }
    }
    generics
}
