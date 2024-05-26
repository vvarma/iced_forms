#![allow(dead_code)]
use derive_builder::Builder;
use iced_form_derive::FormBuilder;

#[derive(Debug, Clone, Builder, FormBuilder)]
struct Config {
    name: String,
    //path: PathBuf,
    seed: usize,
    num: f32,
}

fn main() {}
