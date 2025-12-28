use proc_macro::TokenStream;
use syn::{Attribute, DeriveInput, parse_macro_input};

struct MakeSerdeArgs {
    r#type: syn::Type,
    is_try: bool,
}

impl Default for MakeSerdeArgs {
    fn default() -> Self {
        Self {
            r#type: syn::parse_str::<syn::Type>("u32").unwrap(),
            is_try: false,
        }
    }
}

impl From<&Attribute> for MakeSerdeArgs {
    fn from(attr: &Attribute) -> Self {
        let mut args = Self::default();
        assert!(attr.path().is_ident("make_serde"));
        if attr.meta.require_path_only().is_err() {
            if let syn::Meta::List(list) = &attr.meta {
                list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("try") {
                        args.is_try = true;
                        return Ok(());
                    }
                    if meta.path.is_ident("type") {
                        args.r#type = meta.value().unwrap().parse::<syn::Type>().unwrap();
                        return Ok(());
                    }
                    return Err(meta.error("Unknown attribute"));
                })
                .unwrap();
            }
        }
        args
    }
}

#[proc_macro_derive(MakeSerde, attributes(make_serde))]
pub fn derive_make_serde(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut args: Option<MakeSerdeArgs> = None;
    for ref attr in input.attrs {
        if attr.path().is_ident("make_serde") {
            if args.is_some() {
                panic!("Definitions cannot be repeated multiple times");
            }
            args = Some(attr.into());
        }
    }
    let args = args.unwrap_or_default();
    let t = args.r#type;
    let t_name = match &t {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            path.segments.last().unwrap().ident.to_string()
        }
        _ => panic!(),
    };

    let ser_ident = syn::Ident::new(
        &format!("serialize_{t_name}"),
        proc_macro2::Span::call_site(),
    );

    let der = if args.is_try {
        quote::quote! {
            #t::deserialize(deserializer)?
                .try_into()
                .map_err(serde::de::Error::custom)
        }
    } else {
        quote::quote! {
            #t::deserialize(deserializer).map(Into::into)
        }
    };

    let r#gen = quote::quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.#ser_ident(self.into())
            }
        }

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #der
            }
        }
    };
    TokenStream::from(r#gen)
}

struct SummonFromSummonArgs {
    r#type: syn::Type,
}

impl Default for SummonFromSummonArgs {
    fn default() -> Self {
        Self {
            r#type: syn::parse_str::<syn::Type>("u32").unwrap(),
        }
    }
}

impl From<&Attribute> for SummonFromSummonArgs {
    fn from(attr: &Attribute) -> Self {
        let mut args = Self::default();
        assert!(attr.path().is_ident("summon"));
        if attr.meta.require_path_only().is_err() {
            if let syn::Meta::List(list) = &attr.meta {
                list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("type") {
                        args.r#type = meta.value().unwrap().parse::<syn::Type>().unwrap();
                        return Ok(());
                    }
                    return Err(meta.error("Unknown attribute"));
                })
                .unwrap();
            }
        }
        args
    }
}

#[derive(Default, Clone)]
struct SummonFromOtherArgs {
    clone: bool,
}

impl From<&Attribute> for SummonFromOtherArgs {
    fn from(attr: &Attribute) -> Self {
        let mut args = Self::default();
        assert!(attr.path().is_ident("other"));
        if attr.meta.require_path_only().is_err() {
            if let syn::Meta::List(list) = &attr.meta {
                list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("clone") {
                        args.clone = true;
                        return Ok(());
                    }
                    return Err(meta.error("Unknown attribute"));
                })
                .unwrap();
            }
        }
        args
    }
}

#[proc_macro_derive(SummonFrom, attributes(summon, other))]
pub fn derive_summon_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => panic!(),
    };
    let mut args: Option<SummonFromSummonArgs> = None;
    for ref attr in input.attrs {
        if attr.path().is_ident("summon") {
            if args.is_some() {
                panic!("Definitions cannot be repeated multiple times");
            }
            args = Some(attr.into());
        }
    }
    let args = args.unwrap_or_default();
    let t = args.r#type;
    let (names, exprs, other_name) = data.variants.iter().fold(
        (vec![], vec![], None),
        |(mut acc_n, mut acc_e, other_name), v| {
            for attr in v.attrs.iter() {
                if attr.path().is_ident("other") {
                    if other_name.is_some() {
                        panic!("There should be no more than one other")
                    }
                    let args = SummonFromOtherArgs::from(attr);
                    return (acc_n, acc_e, Some((&v.ident, args.clone())));
                }
            }
            acc_n.push(&v.ident);
            acc_e.push(&v.discriminant.as_ref().unwrap().1);
            (acc_n, acc_e, other_name)
        },
    );
    let from_trait = if other_name.is_some() {
        syn::Ident::new("From", proc_macro2::Span::call_site())
    } else {
        syn::Ident::new("TryFrom", proc_macro2::Span::call_site())
    };
    let from = match other_name.as_ref() {
        Some((n, args)) => {
            let value = if !args.clone {
                quote::quote! {
                    value
                }
            } else {
                quote::quote! {
                    value.clone()
                }
            };
            quote::quote! {
                fn from(value: #t) -> Self {
                    match value {
                        #(
                            #exprs => Self::#names,
                        )*
                        _ => Self::#n(#value)
                    }
                }
            }
        }
        None => {
            quote::quote! {
                type Error = #t;
                fn try_from(value: #t) -> Result<Self, Self::Error> {
                    match value {
                        #(
                            #exprs => Ok(Self::#names),
                        )*
                        _ => Err(value)
                    }
                }
            }
        }
    };
    let into = match other_name {
        Some((n, ..)) => {
            quote::quote! {
                match value {
                    #(
                        #name::#names => #exprs,
                    )*
                    #name::#n(v) => *v,
                }
            }
        }
        None => {
            quote::quote! {
                match value {
                    #(
                        #name::#names => #exprs,
                    )*
                }
            }
        }
    };
    let r#gen = quote::quote! {
        impl #from_trait<#t> for #name {
            #from
        }

        impl From<&#name> for #t {
            fn from(value: &#name) -> #t {
                #into
            }
        }
    };
    TokenStream::from(r#gen)
}
