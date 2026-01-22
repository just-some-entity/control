use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(EnumCount)]
pub fn enum_count(input: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let count = match input.data
    {
        Data::Enum(e) => e.variants.len(),
        _ => panic!("EnumCount can only be derived for enums"),
    };

    TokenStream::from(quote!
    {
        impl #name
        {
            pub const COUNT: usize = #count;
        }
    })
}

#[proc_macro_derive(EnumIndex)]
pub fn enum_index(input: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data
    {
        Data::Enum(e) => e.variants,
        _ => panic!("EnumIndex can only be derived for enums"),
    };

    let arms = variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident;
        quote!
        {
            #name::#ident { .. } => #i
        }
    });

    TokenStream::from(quote!
    {
        impl #name
        {
            pub const fn index(&self) -> usize
            {
                match self
                {
                    #( #arms, )*
                }
            }
        }
    })
}

#[proc_macro_derive(EnumMatch, attributes(enum_match))]
pub fn enum_match(input: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let target = input.attrs.iter()
        .find(|a| a.path().is_ident("enum_match"))
        .expect("missing #[enum_match(TargetEnum)]")
        .parse_args::<syn::Ident>()
        .expect("invalid enum_match argument");

    let source_variants = match &input.data
    {
        Data::Enum(e) => e.variants.iter().map(|v| &v.ident).collect::<Vec<_>>(),
        _ => panic!("EnumMatch only works on enums"),
    };

    let checks = source_variants.iter().map(|v|
    {
        quote!
        {
            let _ = #target::#v;
        }
    });

    TokenStream::from(quote!
    {
        const _: () =
        {
            #( #checks )*
        };
    })
}