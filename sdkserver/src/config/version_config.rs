use std::collections::HashMap;

use common::util::load_or_create_config;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;

const DEFAULT_VERSIONS: &str = include_str!("../../versions.json");

#[derive(Deserialize)]
pub struct VersionConfig {
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub lua_url: String,
    pub lua_version: String,
}

lazy_static! {
    pub static ref INSTANCE: HashMap<String, VersionConfig> = {
        let data = load_or_create_config("versions.json", DEFAULT_VERSIONS);
        from_str(&data).unwrap()
    };
}
