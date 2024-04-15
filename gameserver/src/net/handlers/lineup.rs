use super::*;

pub async fn on_get_all_lineup_data_cs_req(
    session: &PlayerSession,
    _body: &GetAllLineupDataCsReq,
) -> Result<()> {
    let player_info = session.player_info();

    session
        .send(
            CMD_GET_ALL_LINEUP_DATA_SC_RSP,
            GetAllLineupDataScRsp {
                retcode: 0,
                cur_index: 0,
                lineup_list: vec![player_info.lineup.clone()],
            },
        )
        .await
}

pub async fn on_get_cur_lineup_data_cs_req(
    session: &PlayerSession,
    _body: &GetCurLineupDataCsReq,
) -> Result<()> {
    let player_info = session.player_info();

    session
        .send(
            CMD_GET_CUR_LINEUP_DATA_SC_RSP,
            GetCurLineupDataScRsp {
                retcode: 0,
                lineup: Some(player_info.lineup.clone()),
            },
        )
        .await
}

pub async fn on_change_lineup_leader_cs_req(
    session: &PlayerSession,
    body: &ChangeLineupLeaderCsReq,
) -> Result<()> {
    let mut player_info = session.player_info_mut();
    player_info.lineup.leader_slot = body.slot;

    session
        .send(
            CMD_CHANGE_LINEUP_LEADER_SC_RSP,
            ChangeLineupLeaderScRsp {
                slot: body.slot,
                retcode: 0,
            },
        )
        .await
}

pub async fn on_join_lineup_cs_req(session: &PlayerSession, body: &JoinLineupCsReq) -> Result<()> {
    let mut player_info = session.player_info_mut();

    if !(0..4).contains(&body.slot) {
        return session
            .send(
                CMD_JOIN_LINEUP_CS_REQ,
                JoinLineupScRsp {
                    retcode: Retcode::RetLineupInvalidMemberPos as u32,
                },
            )
            .await;
    }

    if player_info
        .lineup
        .avatar_list
        .iter()
        .any(|avatar| avatar.slot == body.slot)
    {
        return session
            .send(
                CMD_JOIN_LINEUP_CS_REQ,
                JoinLineupScRsp {
                    retcode: Retcode::RetLineupAvatarAlreadyInit as u32,
                },
            )
            .await;
    }

    player_info
        .lineup
        .avatar_list
        .push(lineup_avatar(body.base_avatar_id, body.slot));

    player_info.sync_lineup(session).await?;
    session
        .send(CMD_JOIN_LINEUP_SC_RSP, JoinLineupScRsp::default())
        .await
}

pub async fn on_replace_lineup_cs_req(
    session: &PlayerSession,
    body: &ReplaceLineupCsReq,
) -> Result<()> {
    let mut player_info = session.player_info_mut();

    player_info.lineup.avatar_list.clear();
    for slot_info in &body.lineup_slots {
        player_info
            .lineup
            .avatar_list
            .push(lineup_avatar(slot_info.id, slot_info.slot));
    }
    player_info.lineup.leader_slot = body.leader_slot;

    player_info.sync_lineup(session).await?;
    session
        .send(CMD_REPLACE_LINEUP_SC_RSP, ReplaceLineupScRsp::default())
        .await
}

pub async fn on_quit_lineup_cs_req(session: &PlayerSession, body: &QuitLineupCsReq) -> Result<()> {
    let mut player_info = session.player_info_mut();
    player_info
        .lineup
        .avatar_list
        .retain(|avatar| avatar.id != body.base_avatar_id);

    player_info.sync_lineup(session).await?;
    session
        .send(
            CMD_QUIT_LINEUP_SC_RSP,
            QuitLineupScRsp {
                plane_id: body.plane_id,
                is_mainline: !body.is_virtual,
                is_virtual: body.is_virtual,
                base_avatar_id: body.base_avatar_id,
                retcode: 0,
            },
        )
        .await
}

#[must_use]
const fn lineup_avatar(id: u32, slot: u32) -> LineupAvatar {
    LineupAvatar {
        id,
        slot,
        hp: 10000,
        sp: Some(AmountInfo {
            cur_amount: 10000,
            max_amount: 10000,
        }),
        satiety: 100,
        avatar_type: 3,
    }
}
