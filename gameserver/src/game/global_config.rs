use common::util::load_or_create_config;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;

const DEFAULT_GLOBALS: &str = include_str!("../../globals.json");

lazy_static! {
    pub static ref INSTANCE: Globals = {
        let data = load_or_create_config("globals.json", DEFAULT_GLOBALS);
        from_str(&data).unwrap()
    };
}

#[derive(Deserialize)]
pub struct Globals {
    pub lineup: Vec<u32>,
    pub monster_wave_list: Vec<Vec<u32>>,
}
