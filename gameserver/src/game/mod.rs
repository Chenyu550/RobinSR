mod global_config;
mod player_info;

pub use global_config::INSTANCE as globals;
pub use player_info::PlayerInfo;

pub fn init_config() {
    let _globals = &*globals; // this will initialize the static (and create default config)
}
