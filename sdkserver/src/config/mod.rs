mod version_config;

pub use version_config::INSTANCE as versions;

pub fn init_config() {
    tracing::info!("loaded {} version configs", versions.len());
}
