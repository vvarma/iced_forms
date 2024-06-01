#![allow(dead_code)]
use derive_builder::Builder;
use iced::{advanced::Application, executor, Command, Element, Font, Renderer, Settings, Theme};
use iced_form_derive::FormBuilder;

#[derive(Debug, Clone, FormBuilder, PartialEq)]
enum UnitEnum {
    TypeOne,
    TypeTwo,
}

#[derive(Debug, Clone, Builder, FormBuilder)]
struct SomeStruct {
    seed: f32,
}

#[derive(Debug, Clone, FormBuilder)]
enum VarEnum {
    VarT(i32),
    VarTString { name: String, ss: SomeStruct },
    VarTSS(SomeStruct),
}

#[derive(Debug, Clone, Builder, FormBuilder)]
struct SubConfig {
    name: String,
    seed: usize,
    unit_enum: UnitEnum,
    var_enum: VarEnum,
}

#[derive(Debug, Clone, Builder, FormBuilder)]
struct Config {
    name: String,
    //path: PathBuf,
    seed: usize,
    num: f32,
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
    Ok(())
    //App::run(Settings {
    //    default_font: Font::MONOSPACE,
    //    ..Default::default()
    //})
}
