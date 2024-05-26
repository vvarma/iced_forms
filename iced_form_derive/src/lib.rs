use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, Type,
    TypePath,
};

#[proc_macro_derive(FormBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let tokens = match &input.data {
        Data::Struct(data) => derive_for_struct(&input, data),
        Data::Enum(data) => derive_for_enum(&input, data),
        Data::Union(_) => unimplemented!(),
    };
    eprintln!("TOKENS: {}", tokens);
    tokens
}

fn derive_for_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    if data
        .variants
        .iter()
        .all(|f| matches!(f.fields, Fields::Unit))
    {
        derive_for_unit_enum(input, data)
    } else {
        unimplemented!()
    }
}

fn derive_for_unit_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
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

fn derive_for_struct(input: &DeriveInput, data: &DataStruct) -> TokenStream {
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
                Self{
                    builder: ::std::default::Default::default(),
                    #form_default
                }
            }
        }
        impl #form_name{
            #vis fn view_nested(&self)->::iced::Element<#form_message> {
                iced::widget::column![
                    #form_view
                ].into()

            }
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
    TokenStream::from(expanded)
}

fn form_message_name(ident: &Ident) -> Ident {
    format_ident!("{}FormMessage", ident)
}
fn form_name(ident: &Ident) -> Ident {
    format_ident!("{}Form", ident)
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
                    let sub_ty = get_type_ident(ty);
                    let sub_message = form_message_name(&sub_ty);
                    quote_spanned! {f.span()=>#pascal_name(#sub_message)}
                }
            });
            let form_fields = fields.named.iter().filter(|f| !is_bool(&f.ty)).map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                let form_name = form_name(&get_type_ident(ty));
                if is_form_field_type(ty) {
                    quote_spanned! {f.span()=> #name: ::iced_form::form_field::FormField<#ty> }
                } else {
                    quote_spanned! {f.span()=>#name:#form_name }
                }
            });
            let form_default = fields.named.iter().filter(|f| !is_bool(&f.ty)).map(|f|{
                    let name = &f.ident;
                    let title_name = name.clone().unwrap().to_string().to_case(Case::Title);
                    if is_form_field_type(&f.ty) {
                        quote_spanned! {f.span()=>#name: ::iced_form::form_field::FormField::new(#title_name)}
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
                    let sub_ty = get_type_ident(&f.ty);
                    let sub_message = form_message_name(&sub_ty);

                    quote_spanned! {f.span()=>
                        #form_message::#pascal_name(message) => {
                            if let #sub_message::#sub_ty(ref val) = message{
                                self.builder.#name(val.clone());
                            }
                            self.#name.update(message).map(#form_message::#pascal_name)
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

fn get_type_ident(ty: &Type) -> Ident {
    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            if let Some(seg) = path.segments.first() {
                seg.ident.clone()
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

fn is_bool(ty: &Type) -> bool {
    get_type_ident(ty) == "bool"
}

fn is_form_field_type(ty: &Type) -> bool {
    FORM_FIELD_TYPES.contains(&get_type_ident(ty).to_string().as_str())
}

const FORM_FIELD_TYPES: [&'static str; 15] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32",
    "f64", "String",
];
