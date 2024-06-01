# iced_forms
A `proc_macro_derive` implementation to generate boilerplate around building forms. Please have a look at `iced_form/examples/simple.rs` vs `iced_form_derive/examples/simple.rs` to get a feel of what the macro generates given a struct.

## Features
- Support for most primitives using `iced_forms::form_field::FormField<T>` (a wrapper around a `text_field`)
- Support for `bool` using `iced::widgets::toggler`
- Support for Enums without any fields using `iced::widgets::pick_list`
- Nested structs are supported
- Support for Enums with named and un-named fields.

## Planned
- Support for std::chrono - `date_picker`
- Support for PathBuf 
