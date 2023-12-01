use std::collections::HashMap;
use core::fmt;
use crate::bitreader::BitReader;
use crate::structs::net_svc_message::{NetSvcMessage, NetSvcMessageTypes};
use crate::structs::data_manager::DataManager;

use super::netsvc_types::SvcSetPause;

// used a bunch in usermessages
// dont know what these really are
#[derive(Debug, PartialEq, Clone)]
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
// this doesnt need to be a struct but i like it being a struct
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct ServerClass {
    pub datatable_id: i32,
    pub class_name: String,
    pub data_table_name: String,
}

impl ServerClass {
    pub fn new() -> Self {
        Self { datatable_id: 0, class_name: "".to_string(), data_table_name: "".to_string() }
    }
}

// placeholders for now
#[derive(Debug, Clone)]
pub struct StringTable;

impl StringTable {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct VoiceData;

impl VoiceData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct SoundInfo;

#[derive(Debug, Clone)]
pub struct SplitScreenData;

impl SplitScreenData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct EntityMessageData;

impl EntityMessageData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct PacketEntitiesData;

impl PacketEntitiesData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct TempEntitiesData;

impl TempEntitiesData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct MenuData;

impl MenuData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct CmdKeyValuesData;

impl CmdKeyValuesData {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct PaintmapData;

impl PaintmapData {
    pub fn new() -> Self {
        Self
    }
}

// GameEvent stuff
/*
first big thing i implemented for this parser

when you get SvcGameEvent list net/svc message, you save a list of all possible game events 
and what values each of them are supposed to store (by parsing GameEventDescriptors)

later when you get an SvcGameEvent message you look at that list and get the game event at the specified ID

you then look at the values that game event is supposed to have ("keys" field in GameEventDescriptor)

and parse the specified values, populating the "keys" field of the GameEvent struct
*/

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct GameEvent {
    pub descriptor: GameEventDescriptor,
    pub keys: HashMap<String, GameEventKeyTypes> // keys = {"name of value" : value as enum}
}

impl GameEvent {
    pub fn new() -> Self {
        Self { descriptor: GameEventDescriptor::new(), keys: HashMap::new() }
    }
}

#[derive(Debug, Clone)]
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

// new and add_vec3 from https://github.com/lopossumi/Rust-Vectors
// everything else from me
// basically just a vector3 of floats which you can add with other vectors
#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
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
        let mut base_str: String = "                                        ".to_string();
        base_str.replace_range(0..self.x.to_string().len(), &self.x.to_string());
        base_str.replace_range(10..self.y.to_string().len() + 10, &self.y.to_string());
        base_str.replace_range(20..self.z.to_string().len() + 20, &self.z.to_string());
        write!(f, "{}", base_str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut base_str: String = "                              ".to_string();
        base_str.replace_range(0..self.x.to_string().len(), &self.x.to_string());
        base_str.replace_range(10..self.y.to_string().len() + 10, &self.y.to_string());
        write!(f, "{}", base_str)
    }
}

// theres a better way to go about this whole pause checking thing probably
// too bad
pub fn check_for_pause(messages: &Vec<NetSvcMessage>, data_mgr: &mut DataManager) -> bool {
    let data: SvcSetPause = messages.iter().find(|m| {m.msg_type == NetSvcMessageTypes::SvcSetPause}).unwrap().data.clone().into();
    return data.paused;
}

// log2(x) + 1; used a bunch in important places
// i dont use the builtin log2 function bc it only works with floats and makes things messy
pub fn log2_of_x_plus_one(x: i32) -> i32 {
    let mut j = 31;
    // for (j = 31; j >= 0 && (x & (1 << j)) == 0; j--);
    while j >= 0 && (x & (1 << j)) == 0 {
        j -= 1;
    }
    return j + 1;
}

// below is something i shouldve done a long time ago

pub fn bitflags_to_string<B: bitflags::Flags>(names: bitflags::iter::IterNames<B>) -> String {
    let mut flag_str = "".to_string();
    for name in names {
        flag_str.push_str(name.0);
        flag_str.push_str(" | ");
    }
    if flag_str == "" {
        flag_str = "None".to_string();
    } else {
        flag_str = flag_str[..flag_str.len() - 3].to_string();
    }
    
    return flag_str;
}