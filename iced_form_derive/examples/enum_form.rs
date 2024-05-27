use derive_builder::Builder;
use iced_form_derive::FormBuilder;
#[derive(Builder, Clone, Debug, FormBuilder)]
struct Price {
    cost: f32,
    tax: f32,
}

#[derive(Clone, Debug, FormBuilder)]
enum Status {
    Sale(Price),
    Rent { price: Price, min_days: usize },
    NotForSale,
}

fn main() -> iced::Result {
    iced::program("Status Form", StatusForm::update, StatusForm::view)
        .settings(iced::Settings {
            default_font: iced::Font::MONOSPACE,
            ..Default::default()
        })
        .run()
}
