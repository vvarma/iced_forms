#![allow(dead_code)]
use derive_builder::Builder;
use iced::{advanced::Application, executor, Command, Element, Font, Renderer, Settings, Theme};
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

struct App {
    form: StatusForm,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = StatusFormMessage;
    type Renderer = Renderer;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                form: StatusForm::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Config Form".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.form.update(message)
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Self::Renderer> {
        self.form.view()
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        default_font: Font::MONOSPACE,
        ..Default::default()
    })
}
