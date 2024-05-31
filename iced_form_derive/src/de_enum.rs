use crate::{form_message_name, form_name};
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    spanned::Spanned, DataEnum, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Variant,
};

pub fn derive_for_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let form_enum = generate_form_enum(input, data);
    let builders = generate_variant_builder(input, data);
    let messages = generate_form_message(input, data);
    let kinds = generate_variant_kinds(input, data);
    let wrapper = generate_form_wrapper(input, data);
    let expanded = quote! {
        #builders
        #messages
        #form_enum
        #kinds
        #wrapper
    };
    TokenStream::from(expanded)
}

/// For each variant of the enum we create a Unit enum with the `Kind` suffix
/// This is used to first create a `::iced::widget::pick_list` upon whose selection
/// the corresponding Var FormBuilder is used.
fn kind_name(ident: &Ident) -> Ident {
    format_ident!("{}Kind", ident)
}
/// For each variant of the enum (non Unit typed), we create a struct with the `Var` suffix to hold all named/unnamed fields
/// This is then annotated with `FormBuilder` which is handled then by `de_struct.rs`
fn variant_builder_name(var: &Variant) -> Ident {
    format_ident!("{}Var", &var.ident)
}
/// The top level struct that holds the <enum>Kind pick_list and the <enum>EnumFormBuilder which will
/// be the point of interaction if used directly or from an external FormBuilder
fn form_wrapper_name(ident: &Ident) -> Ident {
    form_name(ident)
}
/// The internal enum FormBuilder that has a variant for each variant of the annotated enum
fn form_enum_name(ident: &Ident) -> Ident {
    form_name(&format_ident!("{}Enum", ident))
}
fn form_enum_message(ident: &Ident) -> Ident {
    form_message_name(&format_ident!("{}Enum", ident))
}

fn form_wrapper_message(ident: &Ident) -> Ident {
    form_message_name(ident)
}

fn generate_form_message(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let builder_variants = data
        .variants
        .iter()
        .filter(|var| !matches!(&var.fields, Fields::Unit))
        .map(|var| {
            let builder_variant_field = match &var.fields {
                Fields::Unnamed(_) | Fields::Named(_) => {
                    let builder_name = form_message_name(&variant_builder_name(var));
                    quote_spanned! {var.span()=>(#builder_name)}
                }
                Fields::Unit => unimplemented!(),
            };
            let ident = &var.ident;
            quote_spanned! {var.span()=>
                #ident #builder_variant_field
            }
        });
    let ident = &input.ident;
    let form_message_name = form_enum_message(ident);
    let vis = &input.vis;
    quote_spanned! {input.span()=>
        #[derive(Debug,Clone)]
        #vis enum #form_message_name{
            #(#builder_variants ,)*
            #ident(#ident)
        }
    }
}

fn generate_form_enum(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let builder_variants = data.variants.iter().map(|var| {
        let builder_variant_field = match &var.fields {
            Fields::Unnamed(_) | Fields::Named(_) => {
                let builder_name = form_name(&variant_builder_name(var));
                quote_spanned! {var.span()=>(#builder_name)}
            }
            Fields::Unit => quote! {},
        };
        let ident = &var.ident;
        quote_spanned! {var.span()=>
            #ident #builder_variant_field
        }
    });
    let ident = &input.ident;
    let msg_name = form_enum_message(ident);
    let view_variants = data.variants.iter().map(|var| {
        let ident = &var.ident;
        let ident_str = format!("{}", ident);
        match &var.fields {
            Fields::Named(_) | Fields::Unnamed(_) => quote_spanned! {var.span()=>
                Self::#ident(form)=>form.view_nested().map(#msg_name::#ident)
            },
            Fields::Unit => quote_spanned! {var.span()=>
                Self::#ident => ::iced::widget::text(#ident_str).into()
            },
        }
    });
    let build_variants = data.variants.iter().map(|var| {
        let var_ident = &var.ident;
        match &var.fields {
            Fields::Named(_) | Fields::Unnamed(_) => quote_spanned! {var.span()=>
                Self::#var_ident(form)=>form.build().map(|res|res.build())
            },
            Fields::Unit => quote_spanned! {var.span()=>
                Self::#var_ident => Some(#ident::#var_ident)
            },
        }
    });
    let update_variants = data
        .variants
        .iter()
        .filter(|var| !matches!(&var.fields, Fields::Unit))
        .map(|var| {
            let var_ident = &var.ident;
            match &var.fields {
                Fields::Named(_) | Fields::Unnamed(_) => quote_spanned! {var.span()=>
                    (#msg_name::#var_ident(message), Self::#var_ident(form))=>form.update(message).map(#msg_name::#var_ident)
                },
                Fields::Unit => unimplemented!(),
            }
        });
    let form_name = form_enum_name(ident);
    let vis = &input.vis;
    quote! {
        #vis enum #form_name{
            #(#builder_variants ,)*
        }
        impl #form_name {
        #vis fn view_nested<'a, Theme>(&'a self) -> ::iced::Element<'a, #msg_name, Theme>
        where Theme: ::iced_form::Catalog + 'a
        {
            match self{
                #(#view_variants,)*
            }
        }
        #vis fn build(&self) -> ::std::option::Option<#ident>{
            match self {
                #(#build_variants,)*
            }

        }
        #vis fn view<'a, Theme>(&'a self)-> ::iced::Element<'a, #msg_name, Theme>
        where Theme: ::iced_form::Catalog + 'a
        {
            ::iced::widget::column![
                self.view_nested(),
                ::iced::widget::button("Submit").on_press_maybe(self.build().map(#msg_name::#ident))
            ].into()
        }
        #vis fn update(&mut self, message:#msg_name)-> ::iced::Command<#msg_name>{
            match (message, self){
                #(#update_variants ,)*
                _ => { ::iced::Command::none() }
            }
        }
        }
    }
}
fn generate_variant_builder(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let builders = data.variants.iter().map(|var| match &var.fields {
        Fields::Named(fields) => handle_named_variant(input, var, fields),
        Fields::Unnamed(fields) => handle_unnamed_variant(input, var, fields),
        Fields::Unit => quote! {},
    });
    quote_spanned! {input.span()=>
        #(#builders)*
    }
}
fn handle_named_variant(
    input: &DeriveInput,
    var: &Variant,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let builder_fields = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        let ty = &f.ty;
        quote_spanned! {f.span()=> #field_name:#ty}
    });
    let build_fields = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        quote_spanned! {f.span()=> #field_name: self.#field_name}
    });
    let builder_name = variant_builder_name(var);
    let vis = &input.vis;
    let ident = &input.ident;
    let var_ident = &var.ident;
    quote! {
        #[derive(Clone,Debug,FormBuilder,::derive_builder::Builder)]
        #vis struct #builder_name{
            #(#builder_fields,)*
        }
        impl #builder_name{
            #vis fn build(self)-> #ident{
                #ident::#var_ident{#(#build_fields,)*}
            }
        }
    }
}

fn handle_unnamed_variant(
    input: &DeriveInput,
    var: &Variant,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let builder_fields = fields.unnamed.iter().enumerate().map(|(idx, f)| {
        let field_name = format_ident!("field_{}", idx);
        let ty = &f.ty;
        quote_spanned! {f.span()=> #field_name:#ty}
    });
    let build_fields = fields.unnamed.iter().enumerate().map(|(idx, f)| {
        let field_name = format_ident!("field_{}", idx);
        quote_spanned! {f.span()=> self.#field_name}
    });
    let builder_name = variant_builder_name(var);
    let ident = &input.ident;
    let var_ident = &var.ident;
    let vis = &input.vis;
    quote! {
        #[derive(Clone,Debug,FormBuilder,::derive_builder::Builder)]
        #vis struct #builder_name{
            #(#builder_fields,)*
        }
        impl #builder_name{
            #vis fn build(self)-> #ident{
                #ident::#var_ident(#(#build_fields,)*)
            }
        }
    }
}

fn generate_variant_kinds(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let vis = &input.vis;
    let kind_name = kind_name(ident);
    let kinds = data.variants.iter().map(|var| {
        let var_ident = &var.ident;
        quote_spanned! {var.span()=>
            #var_ident
        }
    });
    quote_spanned! {input.span()=>
        #[derive(Clone, Debug, FormBuilder, PartialEq)]
        #vis enum #kind_name{
            #(#kinds, )*
        }
    }
}

fn generate_form_wrapper(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let vis = &input.vis;
    let kind_name = kind_name(ident);
    let kind_message_name = form_message_name(&kind_name);
    let kind_form_name = form_name(&kind_name);
    let form_enum_name = form_enum_name(ident);
    let form_message_name = form_wrapper_message(ident);
    let form_enum_message = form_enum_message(ident);
    let wrapper_name = form_wrapper_name(ident);

    let kind_form_vars = data.variants.iter().map(|var| {
        let var_ident = &var.ident;
        match &var.fields {
            Fields::Named(_) | Fields::Unnamed(_) => {
                let var_name = form_name(&variant_builder_name(var));
                quote_spanned! {var.span()=>
                    #kind_name::#var_ident=>#form_enum_name::#var_ident(#var_name::default())
                }
            }
            Fields::Unit => {
                quote_spanned! {var.span()=>
                    #kind_name::#var_ident=>#form_enum_name::#var_ident
                }
            }
        }
    });

    quote_spanned! {input.span()=>
        #[derive(Clone,Debug)]
        #vis enum #form_message_name{
            PickList(#kind_message_name),
            Form(#form_enum_message),
            #ident(#ident),
        }
        #[derive(::std::default::Default)]
        #vis struct #wrapper_name{
            pick_list: #kind_form_name,
            form: ::std::option::Option<#form_enum_name>
        }
        impl #wrapper_name {
            #vis fn build(&self)-> ::std::option::Option<#ident>{
                self.form.as_ref().and_then(|val|val.build())
            }
            #vis fn view_nested<'a, Theme>(&'a self)-> ::iced::Element<'a, #form_message_name, Theme>
            where Theme: ::iced_form::Catalog + 'a
            {
                let mut content = ::iced::widget::column![
                    self.pick_list.view_nested().map(#form_message_name::PickList),
                ];
                if let Some(form) = &self.form{
                    content = content.push(form.view_nested().map(#form_message_name::Form)).into()
                }
                content.into()
            }
            #vis fn view<'a, Theme>(&'a self) -> ::iced::Element<'a, #form_message_name, Theme>
            where Theme: ::iced_form::Catalog + 'a
            {
                ::iced::widget::column![
                    self.view_nested(),
                    ::iced::widget::button("Submit").on_press_maybe(self.build().map(#form_message_name::#ident))
                ].into()
            }
            #vis fn update(&mut self, message: #form_message_name) -> ::iced::Command<#form_message_name> {
                match message{
                    #form_message_name::PickList(message)=>{
                        if let #kind_message_name::#kind_name(kind) = &message {
                            self.form = Some(match kind {
                                #(#kind_form_vars,)*
                            });
                        }
                        self.pick_list.update(message).map(#form_message_name::PickList)
                    }
                    #form_message_name::Form(message)=> match self.form.as_mut() {
                        Some(form)=> form.update(message).map(#form_message_name::Form),
                        None => ::iced::Command::none(),
                    }
                    _ => ::iced::Command::none(),
                }
            }
        }
    }
}
