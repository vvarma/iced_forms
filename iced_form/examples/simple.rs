use derive_builder::Builder;
use iced::{
    alignment,
    widget::{button, column, toggler},
    Command, Element,
};
use iced_form::form_field::{self, FormField};

#[derive(Builder, Clone, Debug)]
struct Config {
    name: String,
    //path: PathBuf,
    seed: usize,
    num: f32,
    #[builder(default = "false")]
    enabled: bool,
}

#[derive(Clone, Debug)]
enum Message {
    Name(form_field::Message<String>),
    //Path(form_field::Message<PathBuf>),
    Seed(form_field::Message<usize>),
    Num(form_field::Message<f32>),
    Enabled(bool),
    Config(Config),
}

struct ConfigForm {
    builder: ConfigBuilder,
    name_field: FormField<String>,
    //path_field: FormField<PathBuf>,
    seed_field: FormField<usize>,
    num_field: FormField<f32>,
}
impl Default for ConfigForm {
    fn default() -> Self {
        Self {
            builder: ConfigBuilder::default(),
            name_field: FormField::new("Name"),
            seed_field: FormField::new("Seed"),
            num_field: FormField::new("Num"),
        }
    }
}
impl ConfigForm {
    fn view(&self) -> Element<Message> {
        column![
            self.name_field.view().map(Message::Name),
            //       self.path_field.view().map(Message::Path),
            self.seed_field.view().map(Message::Seed),
            self.num_field.view().map(Message::Num),
            toggler(
                Some("Enabled".to_string()),
                self.builder.enabled.unwrap_or(false),
                Message::Enabled
            )
            .text_alignment(alignment::Horizontal::Left),
            button("Submit").on_press_maybe(self.builder.build().ok().map(Message::Config))
        ]
        .into()
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Name(msg) => {
                if let form_field::Message::Value((name, _)) = &msg {
                    self.builder.name(name.to_string());
                }
                self.name_field.update(msg).map(Message::Name)
            }
            Message::Seed(msg) => {
                if let form_field::Message::Value((seed, _)) = msg {
                    self.builder.seed(seed);
                }
                self.seed_field.update(msg).map(Message::Seed)
            }
            Message::Num(msg) => {
                if let form_field::Message::Value((num, _)) = msg {
                    self.builder.num(num);
                }
                self.num_field.update(msg).map(Message::Num)
            }
            Message::Enabled(val) => {
                self.builder.enabled(val);
                Command::none()
            }
            Message::Config(config) => {
                println!("Got config {:?}", config);
                Command::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::program("Config Form", ConfigForm::update, ConfigForm::view)
        .settings(iced::Settings {
            default_font: iced::Font::MONOSPACE,
            ..Default::default()
        })
        .run()
}
