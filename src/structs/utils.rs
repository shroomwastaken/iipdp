use std::collections::HashMap;
use core::fmt;
use crate::bitreader::BitReader;

// used a bunch in usermessages
// ill need to use this at some point so ill just implement it now
#[derive(PartialEq, Clone)]
pub struct EHandle {
    pub val: i32,
}

impl EHandle {
    pub fn ent_index(&self) -> i32 {
        return self.val & ((1 << 11) - 1)
    }

    pub fn serial(&self) -> i32 {
        return self.val >> 11
    }
}

impl fmt::Display for EHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<ent_index: {}, serial: {}>", self.ent_index(), self.serial())
    }
}

// for NetSetConVar net/svc message
#[derive(Clone)]
pub struct ConVar {
    pub convar_name: String,
    pub convar_value: String,
}

impl ConVar {
    pub fn new() -> Self {
        Self { convar_name: "".to_string(), convar_value: "".to_string() }
    }
}

// for SvcClassInfo
#[derive(Clone)]
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
#[derive(Clone)]
pub struct StringTable;

impl StringTable {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
pub struct VoiceData;

impl VoiceData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
pub struct SoundInfo;

#[derive(Clone)]
pub struct SplitScreenData;

impl SplitScreenData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
pub struct EntityMessageData;

impl EntityMessageData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
pub struct PacketEntitiesData;

impl PacketEntitiesData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
pub struct TempEntitiesData;

impl TempEntitiesData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
pub struct MenuData;

impl MenuData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
pub struct CmdKeyValuesData;

impl CmdKeyValuesData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
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

impl GameEvent {
    pub fn new() -> Self {
        Self { descriptor: GameEventDescriptor::new(), keys: HashMap::new() }
    }
}

#[derive(Clone)]
pub struct GameEventDescriptor {
    pub event_id: i32,
    pub name: String,
    pub keys: HashMap<String, i32> // keys = {"name of value" : type_as_int}
}

impl GameEventDescriptor {
    pub fn new() -> Self {
        Self { event_id: 0, name: String::new(), keys: HashMap::new() }
    }

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

// implementation from https://github.com/lopossumi/Rust-Vectors
#[derive(PartialEq, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn add_vec3(&self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn add_vec_int(&self, other: Vec<i32>) -> Vec3 {
        Vec3 {
            x: self.x + other[0] as f32,
            y: self.y + other[1] as f32,
            z: self.z + other[2] as f32,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}