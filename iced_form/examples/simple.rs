#![allow(dead_code)]
use std::fmt::Display;

use derive_builder::Builder;
use iced::{
    alignment,
    widget::{button, column, toggler},
    Command, Element,
};
use iced_form::form_field::{self, FormField};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum VehicleType {
    TwoWheeler,
    FourWheeler,
    Truck,
}
impl VehicleType {
    const ALL: [Self; 3] = [Self::TwoWheeler, Self::FourWheeler, Self::Truck];
}
impl Display for VehicleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TwoWheeler => "2-wheeler",
                Self::FourWheeler => "4-wheeler",
                Self::Truck => "Truck",
            }
        )
    }
}
#[derive(Builder, Clone, Debug)]
struct Price {
    cost: f32,
    tax: f32,
}

#[derive(Clone, Debug)]
enum Status {
    Sale(Price),
    Rent { price: Price, min_days: usize },
}

#[derive(Clone, Debug)]
enum StatusKind {
    Sale,
    Rent,
}
impl StatusKind {
    fn builder(&self) -> StatusBuilder {
        match self {
            Self::Sale => StatusBuilder::Sale(Default::default()),
            Self::Rent => StatusBuilder::Rent(Default::default()),
        }
    }
}

#[derive(Builder, Clone, Debug)]
struct Rent {
    price: Price,
    min_days: usize,
}

enum StatusBuilder {
    Sale(PriceBuilder),
    Rent(RentBuilder),
}
impl StatusBuilder {
    fn status(&self) -> Option<Status> {
        match self {
            Self::Sale(price_builder) => Some(Status::Sale(price_builder.build().ok()?)),
            Self::Rent(rent_builder) => {
                let rent = rent_builder.build().ok()?;
                Some(Status::Rent {
                    price: rent.price,
                    min_days: rent.min_days,
                })
            }
        }
    }
}

#[derive(Builder, Clone, Debug)]
struct Vehicle {
    name: String,
    weight: f32,
    num_wheels: usize,
    #[builder(default = "false")]
    licensed: bool,
    vehicle_type: VehicleType,
    status: Status,
}

#[derive(Clone, Debug)]
enum Message {
    Name(form_field::Message<String>),
    NumWheels(form_field::Message<usize>),
    Weight(form_field::Message<f32>),
    VehicleType(VehicleType),
    Licensed(bool),
    Status(Status),
    Vehicle(Vehicle),
}

struct VehicleForm {
    builder: VehicleBuilder,
    name: FormField<String>,
    num_wheels: FormField<usize>,
    weight: FormField<f32>,
    selected_vehicle_type: Option<VehicleType>,
}
impl Default for VehicleForm {
    fn default() -> Self {
        Self {
            builder: VehicleBuilder::default(),
            name: FormField::new("Name"),
            num_wheels: FormField::new("Num Wheels"),
            weight: FormField::new("Weight"),
            selected_vehicle_type: None,
        }
    }
}
impl VehicleForm {
    fn view(&self) -> Element<Message> {
        column![
            self.name.view().map(Message::Name),
            self.num_wheels.view().map(Message::NumWheels),
            self.weight.view().map(Message::Weight),
            toggler(
                Some("Licensed".to_string()),
                self.builder.licensed.unwrap_or(false),
                Message::Licensed
            )
            .text_alignment(alignment::Horizontal::Left),
            iced::widget::pick_list(
                VehicleType::ALL,
                self.selected_vehicle_type.clone(),
                Message::VehicleType
            ),
            button("Submit").on_press_maybe(self.builder.build().ok().map(Message::Vehicle))
        ]
        .into()
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Name(msg) => {
                if let form_field::Message::Value((name, _)) = &msg {
                    self.builder.name(name.to_string());
                }
                self.name.update(msg).map(Message::Name)
            }
            Message::NumWheels(msg) => {
                if let form_field::Message::Value((seed, _)) = msg {
                    self.builder.num_wheels(seed);
                }
                self.num_wheels.update(msg).map(Message::NumWheels)
            }
            Message::Weight(msg) => {
                if let form_field::Message::Value((num, _)) = msg {
                    self.builder.weight(num);
                }
                self.weight.update(msg).map(Message::Weight)
            }
            Message::Licensed(val) => {
                self.builder.licensed(val);
                Command::none()
            }
            Message::VehicleType(vehicle_type) => {
                self.builder.vehicle_type(vehicle_type.clone());
                self.selected_vehicle_type = Some(vehicle_type);
                Command::none()
            }
            Message::Status(status) => {
                self.builder.status(status);
                Command::none()
            }
            Message::Vehicle(vehicle) => {
                println!("Got vehicle {:?}", vehicle);
                Command::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::program("Config Form", VehicleForm::update, VehicleForm::view)
        .settings(iced::Settings {
            default_font: iced::Font::MONOSPACE,
            ..Default::default()
        })
        .run()
}
