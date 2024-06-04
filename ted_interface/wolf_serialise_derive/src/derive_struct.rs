use quote::quote;

pub fn derive_wolf_serialise_struct(
    ident: syn::Ident,
    struct_input: syn::DataStruct,
    generics: syn::Generics,
) -> proc_macro::TokenStream {
    let mut generic_params = Vec::new();
    for param in generics.params.iter() {
        generic_params.push(param.clone());
    }
    match struct_input.fields {
        syn::Fields::Named(named) => {
            let mut field_names = Vec::new();
            let mut field_types = Vec::new();
            for field in named.named.iter() {
                field_names.push(field.ident.clone().unwrap());
                field_types.push(field.ty.clone());
            }
            let ret = quote! {
                impl<#(#generic_params),*> wolf_serialise::WolfSerialise for #ident<#(#generic_params),*> where #(#field_types: wolf_serialise::WolfSerialise),*{
                    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()>{
                        #(self.#field_names.wolf_serialise(out_stream)?;)*
                        Ok(())
                    }
                    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R)->std::io::Result<Self>{
                        #(let #field_names = <#field_types>::wolf_deserialise(in_stream)?;)*
                        Ok(#ident{
                            #(#field_names),*
                        })
                    }
                }
            };
            ret.into()
        }
        syn::Fields::Unnamed(unnamed) => {
            let mut field_types = Vec::new();
            let mut numbers = Vec::new();
            let mut number_names = Vec::new();
            for field in unnamed.unnamed.iter() {
                numbers.push(syn::LitInt::new(
                    &format!("{}", field_types.len()),
                    proc_macro2::Span::call_site(),
                ));
                let number_ident_str = format!("member_{}", field_types.len());
                number_names.push(syn::Ident::new(
                    &number_ident_str,
                    proc_macro2::Span::call_site(),
                ));
                field_types.push(field.ty.clone());
            }
            let ret = quote! {
                impl wolf_serialise::WolfSerialise for #ident{
                    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()>{
                        #(self.#numbers.wolf_serialise(out_stream)?;)*
                        Ok(())
                    }
                    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<#ident>{
                        #(let #number_names = <#field_types>::wolf_deserialise(in_stream)?;)*
                        Ok(#ident(#(#number_names),*))
                    }
                }
            };
            ret.into()
        }
        syn::Fields::Unit => {
            let ret = quote! {
                impl wolf_serialise::WolfSerialise for #ident{
                    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W)->io::Result<()>{
                        Ok(())
                    }
                    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<#ident>{
                        Ok(#ident)
                    }
                }
            };
            ret.into()
        }
    }
}
