#![allow(dead_code)]
use derive_builder::Builder;
use iced::{advanced::Application, executor, Command, Element, Font, Renderer, Settings, Theme};
use iced_form_derive::FormBuilder;

#[derive(Clone, Debug, FormBuilder, PartialEq, PartialOrd)]
enum Var {
    Linux,
    Darwin,
}
#[derive(Clone, Debug, Builder, FormBuilder)]
struct SubConfig {
    name: String,
}
#[derive(Debug, Clone, Builder, FormBuilder)]
struct Config {
    name: String,
    //path: PathBuf,
    seed: usize,
    num: f32,
    enabled: bool,
    var: Var,
    sub_config: SubConfig,
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

fn main() -> iced::Result {
    App::run(Settings {
        default_font: Font::MONOSPACE,
        ..Default::default()
    })
}
