use fixed::types::extra::*;
use fixed::*;
use quote::quote;
use syn::parse::*;
use syn::*;
extern crate proc_macro;

struct LitFloatNegative {
    negative: bool,
    lit: LitFloat,
}
impl Parse for LitFloatNegative {
    fn parse(input: ParseStream) -> Result<Self> {
        let negative = <Token![-]>::parse(input).is_ok();
        let lit = LitFloat::parse(input)?;
        Ok(LitFloatNegative { negative, lit })
    }
}
impl LitFloatNegative {
    fn base10_parse(self) -> Result<f64> {
        let as_float: f64 = self.lit.base10_parse()?;
        let res = if self.negative { -as_float } else { as_float };
        Ok(res)
    }
}

macro_rules! create_fixed_macro {
    ($ty: ty, $tyname: ty, $name: ident) => {
        #[proc_macro]
        pub fn $name(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let as_float_lit = parse_macro_input!(tokens as LitFloatNegative);
            let as_float: f64 = as_float_lit
                .base10_parse()
                .expect("unable to parse float literal");
            let as_fixed = <$ty>::from_num(as_float);
            let as_bits = as_fixed.to_bits();
            (quote! {
                <$tyname>::from_bits(#as_bits)
            })
            .into()
        }
    };
}

create_fixed_macro!(
    FixedI64<U32>,
    coords::fixed::FixedI64<coords::fixed::types::extra::U32>,
    fixed_i64_u32
);

#[proc_macro]
pub fn const_pixel_num(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let fixed = proc_macro2::TokenStream::from(fixed_i64_u32(tokens));
    (quote! {
        PixelNum(#fixed)
    })
    .into()
}
