use crate::data::EXCEL_COLLECTION;

use super::*;

pub async fn on_enter_scene_cs_req(session: &PlayerSession, body: &EnterSceneCsReq) -> Result<()> {
    session
        .send(CMD_ENTER_SCENE_SC_RSP, EnterSceneScRsp::default())
        .await?;

    let entrance_config = EXCEL_COLLECTION
        .map_entrance_configs
        .iter()
        .find(|c| c.id == body.entry_id)
        .unwrap();

    let player = session.player_info();
    let enter_scene_by_server = EnterSceneByServerScNotify {
        reason: EnterSceneReason::None.into(),
        lineup: Some(player.lineup.clone()),
        scene: Some(SceneInfo {
            plane_id: entrance_config.plane_id,
            floor_id: entrance_config.floor_id,
            entry_id: entrance_config.id,
            game_mode_type: 2, // TODO: EntranceType -> enum repr(u32)
            ..Default::default()
        }),
    };

    session
        .send(CMD_ENTER_SCENE_BY_SERVER_SC_NOTIFY, enter_scene_by_server)
        .await
}

pub async fn on_get_cur_scene_info_cs_req(
    session: &PlayerSession,
    _body: &GetCurSceneInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_CUR_SCENE_INFO_SC_RSP,
            GetCurSceneInfoScRsp {
                retcode: 0,
                scene: Some(SceneInfo {
                    plane_id: 20101,
                    floor_id: 20101001,
                    entry_id: 2010101,
                    game_mode_type: 2,
                    chhmmbdhjpg: vec![
                        Dhkacjhaoid {
                            state: 1,
                            group_id: 0,
                            entity_list: vec![SceneEntityInfo {
                                group_id: 0,
                                inst_id: 0,
                                entity_id: 0,
                                actor: Some(SceneActorInfo {
                                    avatar_type: 3,
                                    base_avatar_id: 1309,
                                    map_layer: 2,
                                    uid: session.player_uid(),
                                }),
                                motion: Some(MotionInfo {
                                    aomilajjmii: Some(Vector {
                                        bagloppgnpb: 4480,
                                        bemlopmcgch: 19364,
                                        baimdminomk: -550,
                                    }),
                                    eiaoiankefd: Some(Vector {
                                        bagloppgnpb: 4480,
                                        bemlopmcgch: 19364,
                                        baimdminomk: -550,
                                    }),
                                }),
                                ..Default::default()
                            }],
                        },
                        Dhkacjhaoid {
                            state: 1,
                            group_id: 19,
                            entity_list: vec![SceneEntityInfo {
                                group_id: 19,
                                inst_id: 300001,
                                entity_id: 228,
                                prop: Some(ScenePropInfo {
                                    prop_id: 808,
                                    prop_state: 1,
                                    ..Default::default()
                                }),
                                motion: Some(MotionInfo {
                                    aomilajjmii: Some(Vector {
                                        bagloppgnpb: 4480,
                                        bemlopmcgch: 19364,
                                        baimdminomk: -570,
                                    }),
                                    eiaoiankefd: Some(Vector {
                                        bagloppgnpb: 4480,
                                        bemlopmcgch: 19364,
                                        baimdminomk: -570,
                                    }),
                                }),
                                ..Default::default()
                            }],
                        },
                    ],
                    ..Default::default()
                }),
            },
        )
        .await
}
