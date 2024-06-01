mod de_enum;
mod de_struct;
mod de_unit_enum;
use de_struct::derive_for_struct;
use de_unit_enum::derive_for_unit_enum;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::format_ident;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields, Path, Type, TypePath};

#[proc_macro_derive(FormBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(data) => derive_for_struct(&input, data),
        Data::Enum(data) => derive_for_enum(&input, data),
        Data::Union(_) => unimplemented!(),
    }
}

fn derive_for_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    if data
        .variants
        .iter()
        .all(|f| matches!(f.fields, Fields::Unit))
    {
        derive_for_unit_enum(input, data)
    } else {
        de_enum::derive_for_enum(input, data)
    }
}

fn form_message_name(ident: &Ident) -> Ident {
    format_ident!("{}FormMessage", ident)
}
fn form_name(ident: &Ident) -> Ident {
    format_ident!("{}Form", ident)
}

fn get_type_ident(ty: &Type) -> (Path, Ident) {
    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            let mut path = path.clone();
            let ident = match path.segments.pop() {
                Some(pun_pair) => pun_pair.into_value().ident,
                None => unimplemented!(),
            };
            (path, ident)
        }
        _ => unimplemented!(),
    }
}

fn is_bool(ty: &Type) -> bool {
    get_type_ident(ty).1 == "bool"
}

fn is_form_field_type(ty: &Type) -> bool {
    FORM_FIELD_TYPES.contains(&get_type_ident(ty).1.to_string().as_str())
}

const FORM_FIELD_TYPES: [&str; 15] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32",
    "f64", "String",
];
