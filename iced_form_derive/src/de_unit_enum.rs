use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DataEnum, DeriveInput};

use crate::{form_message_name, form_name};

pub fn derive_for_unit_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let vis = &input.vis;
    let name = &input.ident;
    let form_name = form_name(name);
    let form_message = form_message_name(name);
    let title_name = name.clone().to_string().to_case(Case::Title);
    let (num_variants, variants, variant_displays) = gen_variants(data);
    let expanded = quote! {
        impl #name {
            #vis const ALL: [#name;#num_variants] = [
                #variants
            ];
        }
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,
                    "{}",
                    match self {
                        #variant_displays
                    }
                )
            }
        }
        #[derive(Clone, Debug)]
        #vis enum #form_message{
            #name(#name),
        }
        #[derive(::std::default::Default)]
        #vis struct #form_name{
            selected: Option<#name>,
        }
    impl #form_name {
        #vis fn view_nested(&self)->::iced::Element<#form_message> {
            ::iced::widget::row![
                ::iced::widget::text(#title_name),
                ::iced::widget::pick_list(#name::ALL,self.selected.clone(),#form_message::#name)
            ].into()
        }
        #vis fn build(&self) -> ::std::option::Option<#name> {
            self.selected.clone()
        }
        #vis fn view(&self)->::iced::Element<#form_message> {
            self.view_nested()
        }
        #vis fn update(&mut self, message:#form_message)-> ::iced::Command<#form_message> {
            match message {
                #form_message::#name(val)=>{
                    self.selected=Some(val);
                    ::iced::Command::none()
                }
            }
        }
    }
    };
    TokenStream::from(expanded)
}

fn gen_variants(data: &DataEnum) -> (usize, proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let variants = data.variants.iter().map(|v| {
        let name = &v.ident;
        quote_spanned! {v.span()=>Self::#name}
    });
    let variant_displays = data.variants.iter().map(|v| {
        let name = &v.ident;
        let title_name = name.clone().to_string().to_case(Case::Title);
        quote_spanned! {v.span()=>
            Self::#name=>#title_name
        }
    });

    (
        data.variants.len(),
        quote! {#(#variants ,)* },
        quote! {#(#variant_displays ,)* },
    )
}
