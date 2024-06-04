use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn;

/*  Unnamed is an enum variant of the form Variant(A,B,C)
 *  serialise arm:
 *  Enum::Variant(inner1, inner2, ...) => {
 *      (discriminator as discriminator_type).wolf_serialise(out_stream)?;
 *      inner1.wolf_serialise(out_stream)?
 *      inner2.wolf_serialise(out_stream)?
 *      ...
 *  }
 *
 *  deserialise arm:
 *  discriminator => {
 *      let inner1 = Type1::wolf_deserialise(in_stream)?;
 *      let inner2 = Type1::wolf_deserialise(in_stream)?;
 *      ...
 *      Enum::Variant(inner1, inner2, ...)
 *  }
 */

fn unnamed_serialise_arm(
    ident: &syn::Ident,
    variant_ident: &syn::Ident,
    inner_names: &Vec<syn::Ident>,
    discriminator: &syn::LitInt,
) -> TokenStream {
    quote! {
        #ident::#variant_ident(#(#inner_names),*) => {
            #discriminator.wolf_serialise(out_stream)?;
            #(
                #inner_names.wolf_serialise(out_stream)?;
            )*
        }
    }
}

fn unnamed_deserialise_arm(
    ident: &syn::Ident,
    variant_ident: &syn::Ident,
    inner_names: &Vec<syn::Ident>,
    inner_types: &Vec<syn::Type>,
    discriminator: &syn::LitInt,
) -> TokenStream {
    quote! {
        #discriminator => {
            #(
                let #inner_names = #inner_types::wolf_deserialise(in_stream)?;
            )*
            #ident::#variant_ident(#(#inner_names),*)
        }
    }
}

// Unit is a variant like None

fn unit_serialise_arm(
    ident: &syn::Ident,
    variant_ident: &syn::Ident,
    discriminator: syn::LitInt,
) -> TokenStream {
    quote! {
        #ident::#variant_ident => {
            #discriminator.wolf_serialise(out_stream)?;
        }
    }
}

fn unit_deserialise_arm(
    ident: &syn::Ident,
    variant_ident: &syn::Ident,
    discriminator: syn::LitInt,
) -> TokenStream {
    quote! {
        #discriminator => #ident::#variant_ident
    }
}

pub fn derive_wolf_serialise_enum(ident: syn::Ident, input: syn::DataEnum) -> TokenStream {
    let mut serialise_arms = Vec::new();
    let mut deserialise_arms = Vec::new();
    let number_of_variants = input.variants.len();
    let (suffix, discriminator_type) = if number_of_variants <= (std::u8::MAX as usize) {
        ("u8", quote![u8])
    } else if number_of_variants <= (std::u16::MAX as usize) {
        ("u16", quote![u16])
    } else {
        panic!("Error: disgustingly large enum serialisation");
    };
    for (index, variant) in input.variants.iter().enumerate() {
        let discriminator_as_string = format!("{}{}", index, suffix);
        let discriminator_span = Span::call_site();
        let discriminator = syn::LitInt::new(discriminator_as_string.as_str(), discriminator_span);
        match variant.fields {
            syn::Fields::Unnamed(ref unnamed) => {
                let mut inner_names = Vec::new();
                let mut inner_types = Vec::new();
                for (index, field) in unnamed.unnamed.iter().enumerate() {
                    let inner_name_string = format!("_enum_unnamed_inner_{}", index);
                    let inner_name = syn::Ident::new(inner_name_string.as_str(), Span::call_site());
                    inner_names.push(inner_name);
                    inner_types.push(field.ty.clone());
                }
                let serialise_arm =
                    unnamed_serialise_arm(&ident, &variant.ident, &inner_names, &discriminator);
                let deserialise_arm = unnamed_deserialise_arm(
                    &ident,
                    &variant.ident,
                    &inner_names,
                    &inner_types,
                    &discriminator,
                );
                serialise_arms.push(serialise_arm);
                deserialise_arms.push(deserialise_arm);
            }
            syn::Fields::Unit => {
                let serialise_arm =
                    unit_serialise_arm(&ident, &variant.ident, discriminator.clone());
                let deserialise_arm =
                    unit_deserialise_arm(&ident, &variant.ident, discriminator.clone());
                serialise_arms.push(serialise_arm);
                deserialise_arms.push(deserialise_arm);
            }
            _ => panic!("Enum variant type not supported for serialisation"),
        };
    }
    quote! {
        impl wolf_serialise::WolfSerialise for #ident {
            fn wolf_serialise<W: std::io::Write>(self: &Self, out_stream: &mut W) -> std::io::Result<()> {
                match self {
                    #(#serialise_arms),*
                };
                Ok(())
            }
            fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
                let discriminator = #discriminator_type::wolf_deserialise(in_stream)?;
                Ok(match discriminator {
                    #(#deserialise_arms),*
                    , x => return Err(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("{} is not a valid discriminator for {}", discriminator, stringify!(#ident))
                        )
                    ),
                })
            }
        }
    }
}
