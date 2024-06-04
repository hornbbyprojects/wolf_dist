use syn::DeriveInput;
extern crate proc_macro;

mod derive_enum;
mod derive_struct;
use derive_enum::*;
use derive_struct::*;

#[proc_macro_derive(WolfSerialise)]
pub fn derive_wolf_serialise(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse_macro_input!(tokens as DeriveInput);
    let ident = derive_input.ident;
    match derive_input.data {
        syn::Data::Struct(struct_input) => {
            derive_wolf_serialise_struct(ident, struct_input, derive_input.generics.clone())
        }
        syn::Data::Enum(enum_input) => derive_wolf_serialise_enum(ident, enum_input).into(),
        _ => unimplemented!(),
    }
}
