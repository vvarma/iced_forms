use crate::{form_message_name, form_name, get_type_ident, is_bool, is_form_field_type};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{spanned::Spanned, DataStruct, DeriveInput, Fields};

pub fn derive_for_struct(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let vis = &input.vis;
    let name = &input.ident;
    let form_name = form_name(name);
    let form_message = form_message_name(name);
    let builder_name = format_ident!("{}Builder", name);
    let (enum_variants, form_fields, form_default, form_view, form_update) =
        gen_fields(&data.fields, &form_message);
    let expanded = quote! {
        #[derive(Debug,Clone)]
        #vis enum #form_message{
            #enum_variants
            #name(#name),
        }
        #vis struct #form_name{
            builder: #builder_name,
            #form_fields
        }
        impl ::std::default::Default for #form_name {
            fn default() -> Self {
                let def = #name::default();
                Self{
                    builder: ::std::default::Default::default(),
                    #form_default
                }
            }
        }
        impl #form_name{
            #vis fn view_nested<'a, Theme>(&'a self)->::iced::Element<'a, #form_message,Theme>
            where Theme: ::iced_form::Catalog + 'a
            {
                iced::widget::column![
                    #form_view
                ].into()

            }
            #vis fn build(&self)-> ::std::option::Option<#name>{
                self.builder.build().ok()
            }
            #vis fn view<'a, Theme>(&'a self)->iced::Element<'a, #form_message, Theme>
            where Theme: ::iced_form::Catalog + 'a
            {
                let submit = ::iced::widget::button("Submit").on_press_maybe(self.builder.build().ok().map(|val|#form_message::#name(val)));
                iced::widget::column![
                    #form_view
                    submit,
                ].into()
            }
            #vis fn update(&mut self, message:#form_message)-> ::iced::Command<#form_message>{
                match message {
                    #form_update
                    _ => ::iced::Command::none(),
                }
            }
        }
    };
    TokenStream::from(expanded)
}

fn gen_fields(
    fields: &Fields,
    form_message: &proc_macro2::Ident,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    match fields {
        Fields::Named(fields) => {
            let enum_variants = fields.named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                let pascal_name = format_ident!(
                    "{}",
                    name.clone().unwrap().to_string().to_case(Case::Pascal)
                );
                if is_bool(ty) {
                    quote_spanned! {f.span()=> #pascal_name(#ty) }
                } else if is_form_field_type(ty) {
                    quote_spanned! {f.span()=> #pascal_name(iced_form::form_field::Message<#ty>) }
                } else {
                    let (mut sub_path, sub_ty) = get_type_ident(ty);
                    let sub_message = form_message_name(&sub_ty);
                    sub_path.segments.push(sub_message.into());
                    quote_spanned! {f.span()=>#pascal_name(#sub_path)}
                }
            });
            let form_fields = fields.named.iter().filter(|f| !is_bool(&f.ty)).map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                if is_form_field_type(ty) {
                    quote_spanned! {f.span()=> #name: ::iced_form::form_field::FormField<#ty> }
                } else {
                    let (mut form_path, ty_ident) = get_type_ident(ty);
                    form_path.segments.push(form_name(&ty_ident).into());
                    quote_spanned! {f.span()=>#name:#form_path }
                }
            });
            let form_default = fields.named.iter().filter(|f| !is_bool(&f.ty)).map(|f|{
                    let name = &f.ident;
                    let title_name = name.clone().unwrap().to_string().to_case(Case::Title);
                    if is_form_field_type(&f.ty) {
                        quote_spanned! {f.span()=>#name: ::iced_form::form_field::FormField::new(#title_name).with_value(def.#name)}
                    } else{
                        quote_spanned! {f.span()=>#name: ::std::default::Default::default()}
                    }
                });
            let form_view = fields.named.iter().map(|f| {
                let name = &f.ident;
                let pascal_name = format_ident!(
                    "{}",
                    name.clone().unwrap().to_string().to_case(Case::Pascal)
                );
                if is_bool(&f.ty) {
                    let title_name = name.clone().unwrap().to_string().to_case(Case::Title);
                    quote_spanned! {f.span()=>
                        ::iced::widget::toggler(
                            Some(#title_name.to_owned()),
                            self.builder.#name.unwrap_or(false),
                            #form_message::#pascal_name)
                    }
                } else if is_form_field_type(&f.ty){
                    quote_spanned! {f.span()=> self.#name.view().map(#form_message::#pascal_name)}
                } else{
                    quote_spanned! {f.span()=> self.#name.view_nested().map(#form_message::#pascal_name)}
                }
            });
            let form_update = fields.named.iter().map(|f| {
                let name = &f.ident;
                let pascal_name = format_ident!(
                    "{}",
                    name.clone().unwrap().to_string().to_case(Case::Pascal)
                );
                if is_bool(&f.ty) {
                    quote_spanned! {f.span()=>
                        #form_message::#pascal_name(val)=>{
                            self.builder.#name(val);
                            ::iced::Command::none()
                        }
                    }
                } else if is_form_field_type(&f.ty) {
                    quote_spanned! {f.span()=>
                        #form_message::#pascal_name(message)=>{
                            if let ::iced_form::form_field::Message::Value((ref val,_))=message{
                                self.builder.#name(val.clone());
                            }
                            self.#name.update(message).map(#form_message::#pascal_name)
                        }
                    }
                } else {
                    quote_spanned! {f.span()=>
                        #form_message::#pascal_name(message) => {
                            let cmd = self.#name.update(message).map(#form_message::#pascal_name);
                            if let Some(val) = self.#name.build(){
                                self.builder.#name(val);
                            }
                            cmd
                        }
                    }
                }
            });
            (
                quote! {#(#enum_variants ,)*},
                quote! {#(#form_fields ,)*},
                quote! {#(#form_default ,)*},
                quote! {#(#form_view ,)*},
                quote! {#(#form_update ,)*},
            )
        }
        _ => unimplemented!(),
    }
}
