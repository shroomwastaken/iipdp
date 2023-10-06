use crate::structs::utils::GameEventList;

pub struct DataManager {
    pub game_event_list: GameEventList
}

impl DataManager {
    pub fn new() -> Self {
        Self { game_event_list: GameEventList::new() }
    }
}