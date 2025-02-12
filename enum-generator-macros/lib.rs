#![allow(dead_code)]
#![allow(unused_imports, unused_mut, unused_variables)]
use syn::token;
use syn::token::Comma;
use syn::parse::{Parse, ParseStream};
use syn::{ Token, ItemEnum, 
	Attribute, Ident, 
	Generics, Lit, Visibility,
	parse_macro_input, Data, 
	DeriveInput, Variant };
use syn::punctuated::Punctuated;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::{format_ident};
use quote::TokenStreamExt;
use proc_macro::TokenTree;


#[derive(syn_derive::Parse, syn_derive::ToTokens)]
struct GeneratedEnum {
    #[parse(Attribute::parse_outer)]
    #[to_tokens(|tokens, val| tokens.append_all(val))]
    attrs: Vec<Attribute>,

    vis: Visibility,

    enum_token: Token![enum],

    ident: Ident,

    generics: Generics,

    #[syn(braced)]
    brace_token: token::Brace,

    #[syn(in = brace_token)]
    #[parse(Variant::parse, boxed)]
    leading: Box<Variant>,

    #[syn(in = brace_token)]
    template_sep: Token![:],

    #[syn(in = brace_token)]
    template_prefix: Option<Ident>,

    #[syn(in = brace_token)]
    id_place: Token![_],

    #[syn(in = brace_token)]
    template_suffix: Option<Ident>,

    #[syn(in = brace_token)]
    comma: Token![,],

    #[syn(in = brace_token)]
    dots: Token![...],

    #[syn(in = brace_token)]
    #[parse(Variant::parse, boxed)]
    end: Box<Variant>
}


#[proc_macro]
pub fn generate_enum(tokens: TokenStream) -> TokenStream {
    let generated_enum: GeneratedEnum = syn::parse(tokens).expect("Could not parse as GeneratedEnum");
    //let ast: syn::FieldsNamed = syn::parse(tokens).unwrap();
    // let mut generator_function: TokenStream = "pub enum FunctionKey".parse().unwrap();
    let leading_variant = *generated_enum.leading.clone();
    let end_variant = *generated_enum.end.clone();
    if let Some(prefix) = generated_enum.template_prefix.clone() {
        assert!(leading_variant.ident.to_string().starts_with(&prefix.to_string()));
        assert!(end_variant.ident.to_string().starts_with(&prefix.to_string()));
    }

    if let Some(suffix) = generated_enum.template_suffix.clone() {
        assert!(leading_variant.ident.to_string().ends_with(&suffix.to_string()));
        assert!(end_variant.ident.to_string().ends_with(&suffix.to_string()));
    }

    let definition = ItemEnum {
        attrs: generated_enum.attrs,
        vis: generated_enum.vis,
        enum_token: generated_enum.enum_token,
        ident: generated_enum.ident,
        generics: generated_enum.generics,
        brace_token: generated_enum.brace_token,
        variants: make_enum_variants(
            (generated_enum.template_prefix,
            generated_enum.template_suffix),
            leading_variant,
            end_variant)
    };

    // println!("{}", quote! { #definition });
    quote! { #definition }.into()
}


fn make_enum_variants(template: (Option<Ident>, Option<Ident>), leading: Variant, end: Variant) -> Punctuated<Variant, Comma> {
    let mut variants = Punctuated::<Variant, Token![,]>::new();
    variants.push(leading.clone());

    let leading_variant_name = leading.ident.to_string();
    let leading_variant_name = leading_variant_name.as_bytes();

    let prefix = if let Some(prefix) = template.0 {
        prefix.to_string()
    } else {
        "".to_string()
    };

    let suffix = if let Some(suffix) = template.1 {
        suffix.to_string()
    } else {
        "".to_string()
    };


    let starting_char = leading_variant_name[prefix.len()..prefix.len()+1][0];
    let start = unsafe { String::from_utf8_unchecked(
        leading_variant_name[prefix.len()..leading_variant_name.len() - suffix.len()].to_vec())
    };
    let name_generator: Box<dyn Fn(usize) -> String> = if let Ok(starting_number) = start.parse::<usize>() {
        let suffix = suffix.clone();
        let number_sequence_gen = move |i: usize| -> String { format!("{}{}{}", &prefix, &(i+starting_number).to_string(), &suffix) };
        Box::new(number_sequence_gen)
    } else {
        if start.len() != 1 {
            panic!("could not identify templating starting index/ starting char.");
        }

        let suffix = suffix.clone();
        let letter_sequence_gen = move |i: usize|  format!("{}{}{}",
            &prefix, 
            &((starting_char + i as u8) as char).to_string(), 
            &suffix);
        Box::new(letter_sequence_gen)
    };

    let mut i = 1;
    loop {
        let mut variant = leading.clone();
        let variant_name = &name_generator(i);
        //println!("{} : {}", variant_name, end.ident.to_string() );
        let ident = Ident::new(&variant_name, proc_macro2::Span::call_site());
        variant.ident = ident;
        variants.push(variant);
        if variants.len() > 500 {
            panic!("GenerateEnum Attempted to generate enum with more than 500 variants");
        } else if *variant_name == end.ident.to_string() {
            break;
        }
        i += 1;
    }

    variants
}
