use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use iced::{
    widget::{row, text, text_input},
    Command, Element,
};

#[derive(Clone, Debug)]
pub enum Message<T>
where
    T: Clone,
{
    Input {
        input: String,
        invalid_reason: String,
    },
    Value((T, String)),
}

pub struct FormField<T>
where
    T: Clone,
{
    key: String,
    value: Option<T>,
    value_str: String,
    invalid_reason: Option<String>,
}
impl<T> FormField<T>
where
    T: Clone + FromStr + Display + Debug,
{
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            value: None,
            value_str: Default::default(),
            invalid_reason: None,
        }
    }
    pub fn view(&self) -> Element<Message<T>> {
        row!(
            text(&self.key),
            text_input(&self.value_str, &self.value_str).on_input(|val| match val.parse::<T>() {
                Ok(v) => Message::Value((v, val)),
                Err(_) => Message::Input {
                    input: val,
                    invalid_reason: format!("Expected {}", std::any::type_name::<T>())
                },
            })
        )
        .into()
    }
    pub fn update(&mut self, message: Message<T>) -> Command<Message<T>> {
        match message {
            Message::Input {
                input,
                invalid_reason,
            } => {
                self.value_str = input;
                self.invalid_reason = Some(invalid_reason);
            }
            Message::Value((v, val)) => {
                self.value_str = val;
                self.value = Some(v);
                self.invalid_reason = None;
            }
        }
        Command::none()
    }
}
