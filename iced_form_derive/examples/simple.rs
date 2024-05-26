use derive_builder::Builder;
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

fn main() -> iced::Result {
    iced::program("Config Form", ConfigForm::update, ConfigForm::view)
        .settings(iced::Settings {
            default_font: iced::Font::MONOSPACE,
            ..Default::default()
        })
        .run()
}
