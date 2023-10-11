use crate::structs::utils::GameEventList;

pub struct DataManager {
    pub demo_protocol: i32,
    pub network_protocol: i32,
    pub last_packet_tick: i32,
    pub game_event_list: GameEventList
}

impl DataManager {
    pub fn new() -> Self {
        Self { game_event_list: GameEventList::new(), demo_protocol: 0, network_protocol: 0, last_packet_tick: 0 }
    }
}