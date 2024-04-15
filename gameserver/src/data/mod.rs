mod excels;

pub use excels::*;
use lazy_static::lazy_static;
use serde_json::from_str;

pub fn init_assets() {
    tracing::info!("Loaded {} excel tables", EXCEL_COLLECTION.table_count());
}

lazy_static! {
    pub static ref EXCEL_COLLECTION: ExcelCollection = ExcelCollection::new();
}

pub struct ExcelCollection {
    pub map_entrance_configs: Vec<MapEntranceConfig>,
}

impl ExcelCollection {
    fn new() -> Self {
        Self {
            map_entrance_configs: from_str(&load_asset(
                "assets/ExcelOutput/MapEntranceConfig.json",
            ))
            .unwrap(),
        }
    }

    pub fn table_count(&self) -> usize {
        1
    }
}

fn load_asset(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
