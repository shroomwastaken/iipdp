use std::collections::HashMap;

use crate::bitreader::BitReader;

pub struct ConVar {
    pub convar_name: String,
    pub convar_value: String,
}

impl ConVar {
    pub fn new() -> Self {
        Self { convar_name: "".to_string(), convar_value: "".to_string() }
    }
}

pub struct ServerClass {
    pub class_id: i32,
    pub class_name: String,
    pub data_table_name: String,
}

impl ServerClass {
    pub fn new() -> Self {
        Self { class_id: 0, class_name: "".to_string(), data_table_name: "".to_string() }
    }
}

// placeholders for now
pub struct StringTable;

impl StringTable {
    pub fn new() -> Self {
        Self
    }
}

pub struct VoiceData;

impl VoiceData {
    pub fn new() -> Self {
        Self
    }
}

pub struct SoundInfo;

pub struct SplitScreenData;

impl SplitScreenData {
    pub fn new() -> Self {
        Self
    }
}

pub struct UserMessageData;

impl UserMessageData {
    pub fn new() -> Self {
        Self
    }
}

pub struct EntityMessageData;

impl EntityMessageData {
    pub fn new() -> Self {
        Self
    }
}

pub struct PacketEntitiesData;

impl PacketEntitiesData {
    pub fn new() -> Self {
        Self
    }
}

pub struct TempEntitiesData;

impl TempEntitiesData {
    pub fn new() -> Self {
        Self
    }
}

pub struct MenuData;

impl MenuData {
    pub fn new() -> Self {
        Self
    }
}

pub struct CmdKeyValuesData;

impl CmdKeyValuesData {
    pub fn new() -> Self {
        Self
    }
}

pub struct PaintmapData;

impl PaintmapData {
    pub fn new() -> Self {
        Self
    }
}

// GameEvent stuff
#[derive(Clone)]
pub enum GameEventKeyTypes {
    None,
    String(String),
    Float(f32),
    Int32(i32),
    Int16(i32),
    Int8(i32),
    Boolean(bool),
    UInt64(u64),
}

impl GameEventKeyTypes {
    pub fn to_string(self) -> String {
        match self  {
            Self::Boolean(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Int16(value) => value.to_string(),
            Self::Int32(value) => value.to_string(),
            Self::Int8(value) => value.to_string(),
            Self::String(value) => value,
            Self::UInt64(value) => value.to_string(),
            _ => "you arent supposed to be seeing this".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct GameEvent {
    pub descriptor: GameEventDescriptor,
    pub keys: HashMap<String, GameEventKeyTypes> // keys = {"name of value" : value as enum}
}

#[derive(Clone)]
pub struct GameEventDescriptor {
    pub event_id: i32,
    pub name: String,
    pub keys: HashMap<String, i32> // keys = {"name of value" : type_as_int}
}

impl GameEventDescriptor {
    pub fn parse(reader: &mut BitReader) -> Self {
        let event_id = reader.read_int(9);
        let name = reader.read_ascii_string_nulled();
        let mut keys: HashMap<String, i32> = HashMap::new();

        let mut value_type = reader.read_int(3);
        while value_type != 0 {
            keys.entry(reader.read_ascii_string_nulled()).or_insert(value_type);
            
            value_type = reader.read_int(3);
        }

        Self { event_id: event_id, name: name, keys: keys }
    }
}

pub struct GameEventList {
    pub events: i32,
    pub length: i32,
    pub data: Vec<GameEvent>
}

impl GameEventList {
    pub fn new() -> Self {
        let data: Vec<GameEvent> = Vec::new();
        Self { events: 0, length: 0, data: data }
    }
}