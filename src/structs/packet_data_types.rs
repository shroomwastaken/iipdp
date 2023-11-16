use crate::structs::cmd_info::CmdInfo;
use crate::structs::user_cmd_info::UserCmdInfo;
use crate::structs::net_svc_message::NetSvcMessage;
use crate::structs::stringtable::StringTable;
use crate::structs::send_table::SendTable;
use crate::structs::utils::ServerClass;

// just definitions for all the packet types, should move parsing here too

// 'Packet' packet

// probably the single most important packet type
// contains info about the players position as well as net/svc messages
// which have all sorts of data in them

#[derive(Debug)]
pub struct PP {
    pub cmd_info: CmdInfo,
    pub in_sequence: i32,
    pub out_sequence: i32,
    pub size: i32,
    pub messages: Vec<NetSvcMessage>,
}

impl PP {
    pub fn new() -> Self {
        Self { cmd_info: CmdInfo::new(), in_sequence: 0, out_sequence: 0, size: 0, messages: Vec::new() }
    }
}

// synctick packets dont contain any data
#[derive(Debug)]
pub struct SyncTick;

// this packet appears when a game runs a command in the console
#[derive(Debug)]
pub struct ConsoleCmd {
    pub size: i32,
    pub data: String,
}

impl ConsoleCmd {
    pub fn new() -> Self {
        Self { size: 0, data: "".to_string() }
    }
}

// this packet appears on every tick after synctick
// contains info about the players view angles and more (see user_cmd_info.rs)
#[derive(Debug)]
pub struct UserCmd {
    pub cmd: i32,
    pub size: i32,
    pub data: UserCmdInfo,
}

impl UserCmd {
    pub fn new() ->  Self {
        Self { cmd: 0, size: 0, data: UserCmdInfo::new() }
    }
}

// not parsing this yet so this is empty
// contains a lot of datatables which have data about entities
#[derive(Debug)]
pub struct DataTables {
    pub size: i32,
    pub send_table_count: i32,
    pub send_tables: Vec<SendTable>,
    pub class_count: i32,
    pub server_classes: Vec<ServerClass>,
}

impl DataTables {
    pub fn new() -> Self {
        Self { size: 0, send_table_count: 0, send_tables: Vec::new(), class_count: 0, server_classes: Vec::new() }
    }
}

// signifies the last packet, contains no data
#[derive(Debug)]
pub struct Stop;

// this is a really important packet
// only appears once and has data for every string table
// for info about stringtable structure see stringtable.rs
#[derive(Debug, Clone)]
pub struct StringTables {
    pub size: i32,
    pub table_count: i32,
    pub tables: Vec<StringTable>,
}

impl StringTables {
    pub fn new() -> Self {
        Self { size: 0, table_count: 0, tables: Vec::new() }
    }
}
