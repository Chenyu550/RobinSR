pub fn load_or_create_config(path: &str, defaults: &str) -> String {
    if let Ok(data) = std::fs::read_to_string(path) {
        data
    } else {
        std::fs::write(path, defaults).unwrap();
        defaults.to_string()
    }
}
