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
    let (f_v, f_n, f_t, f_d) = data.fields.iter().fold((vec![], vec![], vec![],vec![]), |(mut v, mut n, mut t, mut d), f| {
        d.push(
            f.attrs
                .iter()
                .filter(|a| a.path().is_ident("doc"))
                .collect::<Vec<_>>()
        );
        v.push(&f.vis);
        n.push(&f.ident);
        t.push(&f.ty);
        (v, n, t, d)
    });
    quote::quote! {
        impl #name {
            #(
               #(#f_d)*
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
