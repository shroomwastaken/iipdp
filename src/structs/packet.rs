use crate::structs::packet_data_types as pdt;

// packets are how all of the demos data (except the header) is stored
// for every packet there is its type, its tick, its slot (only on demo protocol 4), and its data (see packet_data_types.rs)

#[derive(Clone, Copy, PartialEq)]
pub enum PacketType {
    Unknown = 0,
    SignOn = 1,
    Packet = 2,
    SyncTick = 3,
    ConsoleCmd = 4,
    UserCmd = 5,
    DataTables = 6,
    Stop = 7,
    StringTables = 8,
}

impl PacketType {
    pub fn from_int(value: i32) -> Self {
        match value {
            1 => PacketType::SignOn,
            2 => PacketType::Packet,
            3 => PacketType::SyncTick,
            4 => PacketType::ConsoleCmd,
            5 => PacketType::UserCmd,
            6 => PacketType::DataTables,
            7 => PacketType::Stop,
            8 => PacketType::StringTables,
            _ => PacketType::Unknown,
        }
    }
}

pub enum PacketDataType {
    Unknown,
    Packet(pdt::PP),
    SyncTick(pdt::SyncTick),
    ConsoleCmd(pdt::ConsoleCmd),
    UserCmd(pdt::UserCmd),
    DataTables(pdt::DataTables),
    Stop(pdt::Stop),
    StringTables(pdt::StringTables),
}

// all the into<> are required to later extrapolate the data from the enum
impl Into<pdt::PP> for PacketDataType {
    fn into(self) -> pdt::PP {
        match self {
            PacketDataType::Packet(value) => value,
            _ => panic!("what the hell are you trying to do???"),
        }
    }
}

impl Into<pdt::ConsoleCmd> for PacketDataType {
    fn into(self) -> pdt::ConsoleCmd {
        match self {
            PacketDataType::ConsoleCmd(value) => value,
            _ => panic!("what the hell are you trying to do???"),
        }
    }
}

impl Into<pdt::UserCmd> for PacketDataType {
    fn into(self) -> pdt::UserCmd {
        match self {
            PacketDataType::UserCmd(value) => value,
            _ => panic!("what the hell are you trying to do???"),
        }
    }
}

impl Into<pdt::DataTables> for PacketDataType {
    fn into(self) -> pdt::DataTables {
        match self {
            PacketDataType::DataTables(value) => value,
            _ => panic!("what the hell are you trying to do???"),
        }        
    }
}

impl Into<pdt::StringTables> for PacketDataType {
    fn into(self) -> pdt::StringTables {
        match self {
            PacketDataType::StringTables(value) => value,
            _ => panic!("what the hell are you trying to do???"),
        }        
    }
}

pub struct Packet {
    pub packet_type: PacketType,
    pub tick: i32,
    pub slot: Option<i32>,
    pub data: PacketDataType
}

impl Packet {
    pub fn new() -> Self {
        Self { packet_type: PacketType::Unknown, tick: 0, slot: None, data: PacketDataType::Unknown }
    }
}
