use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(BuildBuilder)]
pub fn derive_build_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Not struct"),
    };
    let (f_v, f_n, f_t) = data.fields.iter().fold((vec![], vec![], vec![]), |(mut v, mut n, mut t), f| {
        v.push(&f.vis);
        n.push(&f.ident);
        t.push(&f.ty);
        (v, n, t)
    });
    quote::quote! {
        impl #name {
            #(
               #f_v fn #f_n(self, #f_n: #f_t) -> Self {
                    Self {
                        #f_n,
                        ..self
                    }
                }
            )*
        }
    }.into()
}
