extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};

use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Error, Field, Fields,
    GenericParam, Generics, Ident,
};

use quote::{quote, quote_spanned};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span2::call_site(), $string)
            .to_compile_error()
            .into();
    };
}

#[proc_macro_derive(Transmittable)]
pub fn derive_transmittable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ref name = input.ident;

    let mut parse_struct_res = match parse_struct(&input) {
        Ok(parse_struct_res) => parse_struct_res,
        Err(e) => return e,
    };

    // We take mutable ref to the fields of struct because
    // they need to be used inside quote! macro.
    // Struct members can't be interpolated directly in quote!
    let ref mut assert_transmittable_bound_on_fields = parse_struct_res.transmittable_bounds;
    let ref mut serialize_funcs = parse_struct_res.serialize_funcs;
    let ref mut deserialize_funcs = parse_struct_res.deserialize_funcs;
    let ref mut struct_maker = parse_struct_res.struct_maker;

    let generics = add_trait_bound(input.generics, "Transmittable");

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Serializable for #name #ty_generics #where_clause {
            fn serialize_append(&self, mut v: std::vec::Vec<u8>)
                -> std::result::Result<std::vec::Vec<u8>, std::boxed::Box<dyn std::error::Error>>
            {
                fn assert_impl_transmittable<T: ?Sized + Transmittable>() {}
                #assert_transmittable_bound_on_fields
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

struct ParseStructRes {
    transmittable_bounds: TokenStream2,
    serialize_funcs: TokenStream2,
    deserialize_funcs: TokenStream2,
    struct_maker: TokenStream2,
}

impl ParseStructRes {
    fn new() -> ParseStructRes {
        ParseStructRes {
            transmittable_bounds: TokenStream2::new(),
            serialize_funcs: TokenStream2::new(),
            deserialize_funcs: TokenStream2::new(),
            struct_maker: TokenStream2::new(),
        }
    }
}

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

fn gen_code_for_fields(parse_struct_res: &mut ParseStructRes, field: &Field, field_no: usize) {
    // This closure does the common tasks of named and unnamed fields.
    // field_type :- Type of the field
    // field_name :- Name of the field in case of named fields
    // field_no   :- Position of the field. (useful for tuple structs, e.g., self.0, self.1 etc.)

    let ref field_type = field.ty;
    let ref field_name = field.ident;

    parse_struct_res
        .transmittable_bounds
        .extend(quote_spanned! {field.span()=>
            assert_impl_transmittable::<#field_type>();
        });

    if let Some(field_name) = field_name {
        parse_struct_res
            .serialize_funcs
            .extend(quote_spanned! {field.span()=>
                let v = self.#field_name.serialize_append(v)?;
            });

        parse_struct_res
            .deserialize_funcs
            .extend(quote_spanned! {field.span()=>
                let (#field_name, bytes_parsed)
                    = <#field_type as Deserializable>::deserialize(&data[start..])?;
                start += bytes_parsed;
            });

        parse_struct_res
            .struct_maker
            .extend(quote_spanned! {field.span()=>
                #field_name,
            });
    } else {
        parse_struct_res
            .serialize_funcs
            .extend(quote_spanned! {field.span()=>
                let v = self.#field_no.serialize_append(v)?;
            });

        parse_struct_res
            .deserialize_funcs
            .extend(quote_spanned! {field.span()=>
                let (deserialized_#field_no, bytes_parsed)
                    = <#field_type as Deserializable>::deserialize(&data[start..])?;
                start += bytes_parsed;
            });

        parse_struct_res
            .struct_maker
            .extend(quote_spanned! {field.span()=>
                deserialized_#field_no,
            });
    }
}

fn add_trait_bound(mut generics: Generics, bound: &str) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = param {
            let bound_ident = Ident::new(bound, type_param.span());
            type_param.bounds.push(parse_quote!(#bound_ident));
        }
    }
    generics
}
