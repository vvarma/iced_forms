use iced::widget::{button, pick_list, text, text_input, toggler};

pub mod form_field;

#[derive(Default)]
pub struct Style {}
pub trait Catalog:
    text::Catalog + text_input::Catalog + toggler::Catalog + button::Catalog + pick_list::Catalog
{
    /// The item class of this [`Catalog`].
    type Class<'a>;

    /// The default class produced by this [`Catalog`].
    fn default<'a>() -> <Self as Catalog>::Class<'a>;
    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
impl Catalog for iced::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(|_| Style::default())
    }
    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}
