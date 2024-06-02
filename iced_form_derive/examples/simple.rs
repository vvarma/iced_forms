#![allow(dead_code)]
use derive_builder::Builder;
use iced::{advanced::Application, executor, Command, Element, Font, Renderer, Settings, Theme};
use iced_form_derive::FormBuilder;

#[derive(Clone, Debug, FormBuilder, PartialEq, PartialOrd, Default)]
enum Var {
    #[default]
    Linux,
    Darwin,
}
#[derive(Clone, Debug, Builder, FormBuilder)]
#[builder(default)]
struct SubConfig {
    name: String,
}
impl Default for SubConfig {
    fn default() -> Self {
        Self {
            name: "sub_config".to_string(),
        }
    }
}
#[derive(Debug, Clone, Builder, FormBuilder)]
#[builder(default)]
struct Config {
    name: String,
    //path: PathBuf,
    seed: usize,
    num: f32,
    enabled: bool,
    var: Var,
    sub_config: SubConfig,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            name: "Config".to_string(),
            seed: 42,
            num: std::f32::consts::PI,
            enabled: true,
            var: Default::default(),
            sub_config: Default::default(),
        }
    }
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
        if let ConfigFormMessage::Config(config) = &message {
            println!("{:?}", config);
        }
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
