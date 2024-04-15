use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct MapEntranceConfig {
    #[serde(rename = "BeginMainMissionIDList")]
    pub begin_main_mission_idlist: Vec<u32>,
    pub entrance_type: String,
    #[serde(rename = "FinishMainMissionIDList")]
    pub finish_main_mission_idlist: Vec<u32>,
    #[serde(rename = "FinishSubMissionIDList")]
    pub finish_sub_mission_idlist: Vec<u32>,
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
}
