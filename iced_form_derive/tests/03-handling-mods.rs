#![allow(dead_code)]
use derive_builder::Builder;
use iced::{advanced::Application, executor, Command, Element, Renderer, Theme};
use iced_form_derive::FormBuilder;

mod vehicle {
    use iced_form_derive::FormBuilder;

    #[derive(Clone, Debug, FormBuilder, PartialEq)]
    pub enum Fuel {
        Petrol,
        Diesel,
    }
    #[derive(Clone, Debug, FormBuilder)]
    pub enum Config {
        Car { fuel: Fuel, used: bool },
        Bus,
        HorseCart { driver: String },
    }
}

#[derive(Clone, Debug, Builder, FormBuilder)]
struct Config {
    vehicle_config: vehicle::Config,
}

struct App {
    form: ConfigForm,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = ConfigFormMessage;
    type Renderer = Renderer;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                form: ConfigForm::default(),
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

fn main() {}
