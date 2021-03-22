extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{
    Span as Span2,
    TokenStream as TokenStream2,
};

use syn::{
    Data,
    DeriveInput,
    Error,
    Field,
    Fields,
    GenericParam,
    Generics,
    Ident,
    parse_macro_input,
    parse_quote,
    spanned::Spanned,
    Type,
};

use quote::{
    quote,
    quote_spanned,
//    format_ident,
};

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
    let name = input.ident;

    let data = input.data;

    let mut assert_transmittable_bound_on_fields = TokenStream2::new();
    let mut serialize_fields_funcs = TokenStream2::new();
    let mut deserialize_fields_funcs = TokenStream2::new();
    let mut struct_fields = TokenStream2::new();
    let struct_fields_wrapper;

    // This closure does the common tasks of named and unnamed fields.
    // field_type :- Type of the field
    // field_name :- Name of the field in case of named fields
    // field_no   :- Position of the field. (useful for tuple structs, e.g., self.0, self.1 etc.)
    let mut gen_code_for_fields = |field: &Field, field_no: usize| {
        let ref field_type = field.ty;
        let ref field_name = field.ident;

        assert_transmittable_bound_on_fields.extend(quote_spanned! {field.span()=>
            assert_impl_transmittable::<#field_type>();
        });

        if let Some(field_name) = field_name {
            serialize_fields_funcs.extend(quote_spanned! {field.span()=>
                let v = self.#field_name.serialize_append(v)?;
            });
            
            deserialize_fields_funcs.extend(quote_spanned! {field.span()=>
                let (#field_name, bytes_parsed)
                    = <#field_type as Deserializable>::deserialize(&data[start..])?;
                start += bytes_parsed;
            });

            struct_fields.extend(quote_spanned! {field.span()=>
                #field_name,
            });
        } else {
            serialize_fields_funcs.extend(quote_spanned! {field.span()=>
                let v = self.#field_no.serialize_append(v)?;
            });

            deserialize_fields_funcs.extend(quote_spanned! {field.span()=>
                let (deserialized_#field_no, bytes_parsed)
                    = <#field_type as Deserializable>::deserialize(&data[start..])?;
                start += bytes_parsed;
            });

            struct_fields.extend(quote_spanned! {field.span()=>
                deserialized_#field_no,
            });
        }
    };

    match data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields_named) => {
                    for (field_no, field) in fields_named.named.iter().enumerate() {
                        gen_code_for_fields(field, field_no);
                    }

                    struct_fields_wrapper = quote_spanned! {fields_named.span()=>
                        #name {
                            #struct_fields
                        }
                    };
                },
                Fields::Unnamed(fields_unnamed) => {
                    for (field_no, field) in fields_unnamed.unnamed.iter().enumerate() {
                        gen_code_for_fields(field, field_no);
                    }

                    struct_fields_wrapper = quote_spanned! {fields_unnamed.span()=>
                        #name(#struct_fields)
                    };
                },
                Fields::Unit => {
                    struct_fields_wrapper = quote! {
                        #name
                    };
                },
            }
        },
        _ => return derive_error!("#[derive(Transmittable)] only works with struct"),
    }

    let generics = add_trait_bound(input.generics, "Transmittable");

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
 
    let expanded = quote! {
        impl #impl_generics Serializable for #name #ty_generics #where_clause {
            fn serialize_append(&self, mut v: std::vec::Vec<u8>) 
                -> std::result::Result<std::vec::Vec<u8>, std::boxed::Box<dyn std::error::Error>>
            {
                fn assert_impl_transmittable<T: ?Sized + Transmittable>() {}
                #assert_transmittable_bound_on_fields
                #serialize_fields_funcs
                Ok(v)
            }
        }

        impl #impl_generics Deserializable for #name #ty_generics #where_clause {
            fn deserialize(data: &[u8]) 
                -> std::result::Result<(Self, usize), std::boxed::Box<dyn std::error::Error>> 
            {
                let mut start: usize = 0;
                #deserialize_fields_funcs
                Ok((#struct_fields_wrapper, start))
            }
        }

        impl #impl_generics Transmittable for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
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
