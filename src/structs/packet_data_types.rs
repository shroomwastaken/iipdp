use crate::structs::cmd_info::CmdInfo;
use crate::structs::user_cmd_info::UserCmdInfo;
use crate::structs::net_svc_message::NetSvcMessage;
use crate::structs::stringtable::StringTable;

// 'Packet' packet
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
pub struct SyncTick;

pub struct ConsoleCmd {
    pub size: i32,
    pub data: String,
}

impl ConsoleCmd {
    pub fn new() -> Self {
        Self { size: 0, data: "".to_string() }
    }
}

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
pub struct DataTables {
    pub size: i32,
}

impl DataTables {
    pub fn new() -> Self {
        Self { size: 0 }
    }
}

// signifies the last packet, contains no data
pub struct Stop;

// not parsing this yet either
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
