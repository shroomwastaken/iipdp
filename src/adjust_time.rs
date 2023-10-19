// there will be more here when i add support for more games/mods
// for now just p1 wakeup and glados death :)

use crate::structs::{net_svc_message::NetSvcMessage, data_manager::DataManager, netsvc_types::SvcFixAngle,
    net_svc_message::NetSvcMessageTypes::SvcFixAngle as fix_angle_type
};

pub fn try_adjust_for_wakeup(messages: &Vec<NetSvcMessage>, tick: i32, data_mgr: &mut DataManager) {
    let data: SvcFixAngle = messages.iter().find(|m| {m.msg_type == fix_angle_type}).unwrap().data.clone().into();
    if data.angle == vec![0f32, 189.99756f32, 0f32] {
        data_mgr.adj_start_tick = tick + 1; // plus one because yes
    }
}

pub fn try_adjust_for_glados_death(command: &String, tick: i32, data_mgr: &mut DataManager) {
    if command == "startneurotoxins 99999" {
        data_mgr.adj_end_tick = tick + 1 // again plus one because yes
    }
}
