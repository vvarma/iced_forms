use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields};

#[proc_macro_derive(FormBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let vis = input.vis;
    let name = input.ident;
    let form_name = format_ident!("{}Form", name);
    let form_message = format_ident!("{}FormMessage", name);
    let builder_name = format_ident!("{}Builder", name);
    let (enum_variants, form_fields, form_default, form_view, form_update) =
        gen_fields(&input.data, &form_message);
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
                Self{
                    builder: ::std::default::Default::default(),
                    #form_default
                }
            }
        }
        impl #form_name{
            #vis fn view(&self)->iced::Element<#form_message>{
                let submit = ::iced::widget::button("Submit").on_press_maybe(self.builder.build().ok().map(|val|#form_message::#name(val)));
                iced::widget::column![
                    #form_view
                    submit,
                ].into()
            }
            #vis fn update(&mut self, message:#form_message)->iced::Command<#form_message>{
                match message {
                    #form_update
                    _ => ::iced::Command::none(),
                }
            }
        }
    };
    let tokens = TokenStream::from(expanded);
    eprintln!("TOKENS: {}", tokens);
    tokens
}

fn gen_fields(
    data: &Data,
    form_message: &proc_macro2::Ident,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let enum_variants = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    let pascal_name = format_ident!(
                        "{}",
                        name.clone().unwrap().to_string().to_case(Case::Pascal)
                    );
                    quote_spanned! {f.span()=> #pascal_name(iced_form::form_field::Message<#ty>) }
                });
                let form_fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span()=> #name: ::iced_form::form_field::FormField<#ty> }
                });
                let form_default = fields.named.iter().map(|f|{
                    let name = &f.ident;
                    let title_name = name.clone().unwrap().to_string().to_case(Case::Title);
                    quote_spanned! {f.span()=>#name: ::iced_form::form_field::FormField::new(#title_name)}
                });
                let form_view = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let pascal_name = format_ident!(
                        "{}",
                        name.clone().unwrap().to_string().to_case(Case::Pascal)
                    );
                    quote_spanned! {f.span()=> self.#name.view().map(#form_message::#pascal_name)}
                });
                let form_update = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let pascal_name = format_ident!(
                        "{}",
                        name.clone().unwrap().to_string().to_case(Case::Pascal)
                    );
                    quote_spanned! {f.span()=>
                        #form_message::#pascal_name(message)=>{
                            if let ::iced_form::form_field::Message::Value((ref val,_))=message{
                                self.builder.#name(val.clone());
                            }
                            self.#name.update(message).map(#form_message::#pascal_name)
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
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
