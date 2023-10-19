use std::fs::File;
use std::io::Write;

use crate::structs::netsvc_types::{
    NetNop, NetDisconnect, NetFile, NetSetConVar, NetSignonState, NetSplitScreenUser, NetStringCmd, NetTick, SvcBspDecal, SvcClassInfo, SvcCmdKeyValues,
    SvcCreateStringTable, SvcCrosshairAngle, SvcEntityMessage, SvcFixAngle, SvcGameEvent, SvcGameEventList, SvcGetCvarValue, SvcMenu, SvcPacketEntities,
    SvcPaintmapData, SvcPrefetch, SvcPrint, SvcSendTable, SvcServerInfo, SvcSetPause, SvcSetView, SvcSounds, SvcSplitScreen, SvcTempEntities, SvcUpdateStringTable,
    SvcUserMessage, SvcVoiceData, SvcVoiceInit
};

use crate::bitreader::BitReader;
use crate::structs::net_svc_message::NetSvcMessageTypes as nsmt;
use crate::structs::net_svc_message::NetSvcMessageDataTypes as nsmdt;
use crate::structs::netsvc_types as nt;
use crate::structs::utils::GameEventList;
use crate::structs::data_manager::DataManager;
use crate::structs::user_message::write_usermsg_data_to_file;

pub enum NetSvcMessageDataTypes {
    Unknown,
    NetNop(NetNop),
    NetDisconnect(NetDisconnect),
    NetFile(NetFile),
    NetSplitScreenUser(NetSplitScreenUser),
    NetTick(NetTick),
    NetStringCmd(NetStringCmd),
    NetSetConVar(NetSetConVar),
    NetSignonState(NetSignonState),
    SvcPrint(SvcPrint),
    SvcServerInfo(SvcServerInfo),
    SvcSendTable(SvcSendTable),
    SvcClassInfo(SvcClassInfo),
    SvcSetPause(SvcSetPause),
    SvcCreateStringTable(SvcCreateStringTable),
    SvcUpdateStringTable(SvcUpdateStringTable),
    SvcVoiceInit(SvcVoiceInit),
    SvcVoiceData(SvcVoiceData),
    SvcSounds(SvcSounds),
    SvcSetView(SvcSetView),
    SvcFixAngle(SvcFixAngle),
    SvcCrosshairAngle(SvcCrosshairAngle),
    SvcBspDecal(SvcBspDecal),
    SvcSplitScreen(SvcSplitScreen),
    SvcUserMessage(SvcUserMessage),
    SvcEntityMessage(SvcEntityMessage),
    SvcGameEvent(SvcGameEvent),
    SvcPacketEntities(SvcPacketEntities),
    SvcTempEntities(SvcTempEntities),
    SvcPrefetch(SvcPrefetch),
    SvcMenu(SvcMenu),
    SvcGameEventList(SvcGameEventList),
    SvcGetCvarValue(SvcGetCvarValue),
    SvcCmdKeyValues(SvcCmdKeyValues),
    SvcPaintmapData(SvcPaintmapData),
}

impl Into<NetDisconnect> for NetSvcMessageDataTypes {
    fn into(self) -> NetDisconnect {
        match self {
            NetSvcMessageDataTypes::NetDisconnect(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<NetFile> for NetSvcMessageDataTypes {
    fn into(self) -> NetFile {
        match self {
            NetSvcMessageDataTypes::NetFile(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<NetSetConVar> for NetSvcMessageDataTypes {
    fn into(self) -> NetSetConVar {
        match self {
            NetSvcMessageDataTypes::NetSetConVar(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<NetSignonState> for NetSvcMessageDataTypes {
    fn into(self) -> NetSignonState {
        match self {
            NetSvcMessageDataTypes::NetSignonState(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<NetSplitScreenUser> for NetSvcMessageDataTypes {
    fn into(self) -> NetSplitScreenUser {
        match self {
            NetSvcMessageDataTypes::NetSplitScreenUser(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<NetStringCmd> for NetSvcMessageDataTypes {
    fn into(self) -> NetStringCmd {
        match self {
            NetSvcMessageDataTypes::NetStringCmd(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<NetTick> for NetSvcMessageDataTypes {
    fn into(self) -> NetTick {
        match self {
            NetSvcMessageDataTypes::NetTick(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcBspDecal> for NetSvcMessageDataTypes {
    fn into(self) -> SvcBspDecal {
        match self {
            NetSvcMessageDataTypes::SvcBspDecal(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcClassInfo> for NetSvcMessageDataTypes {
    fn into(self) -> SvcClassInfo {
        match self {
            NetSvcMessageDataTypes::SvcClassInfo(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcCmdKeyValues> for NetSvcMessageDataTypes {
    fn into(self) -> SvcCmdKeyValues {
        match self {
            NetSvcMessageDataTypes::SvcCmdKeyValues(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcCreateStringTable> for NetSvcMessageDataTypes {
    fn into(self) -> SvcCreateStringTable {
        match self {
            NetSvcMessageDataTypes::SvcCreateStringTable(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcCrosshairAngle> for NetSvcMessageDataTypes {
    fn into(self) -> SvcCrosshairAngle {
        match self {
            NetSvcMessageDataTypes::SvcCrosshairAngle(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcGetCvarValue> for NetSvcMessageDataTypes {
    fn into(self) -> SvcGetCvarValue {
        match self {
            NetSvcMessageDataTypes::SvcGetCvarValue(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcEntityMessage> for NetSvcMessageDataTypes {
    fn into(self) -> SvcEntityMessage {
        match self {
            NetSvcMessageDataTypes::SvcEntityMessage(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcFixAngle> for NetSvcMessageDataTypes {
    fn into(self) -> SvcFixAngle {
        match self {
            NetSvcMessageDataTypes::SvcFixAngle(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcGameEvent> for NetSvcMessageDataTypes {
    fn into(self) -> SvcGameEvent {
        match self {
            NetSvcMessageDataTypes::SvcGameEvent(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcGameEventList> for NetSvcMessageDataTypes {
    fn into(self) -> SvcGameEventList {
        match self {
            NetSvcMessageDataTypes::SvcGameEventList(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcMenu> for NetSvcMessageDataTypes {
    fn into(self) -> SvcMenu {
        match self {
            NetSvcMessageDataTypes::SvcMenu(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcPacketEntities> for NetSvcMessageDataTypes {
    fn into(self) -> SvcPacketEntities {
        match self {
            NetSvcMessageDataTypes::SvcPacketEntities(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcPaintmapData> for NetSvcMessageDataTypes {
    fn into(self) -> SvcPaintmapData {
        match self {
            NetSvcMessageDataTypes::SvcPaintmapData(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcPrefetch> for NetSvcMessageDataTypes {
    fn into(self) -> SvcPrefetch {
        match self {
            NetSvcMessageDataTypes::SvcPrefetch(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcPrint> for NetSvcMessageDataTypes {
    fn into(self) -> SvcPrint {
        match self {
            NetSvcMessageDataTypes::SvcPrint(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcSendTable> for NetSvcMessageDataTypes {
    fn into(self) -> SvcSendTable {
        match self {
            NetSvcMessageDataTypes::SvcSendTable(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcServerInfo> for NetSvcMessageDataTypes {
    fn into(self) -> SvcServerInfo {
        match self {
            NetSvcMessageDataTypes::SvcServerInfo(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcSetPause> for NetSvcMessageDataTypes {
    fn into(self) -> SvcSetPause {
        match self {
            NetSvcMessageDataTypes::SvcSetPause(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcSetView> for NetSvcMessageDataTypes {
    fn into(self) -> SvcSetView {
        match self {
            NetSvcMessageDataTypes::SvcSetView(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcSounds> for NetSvcMessageDataTypes {
    fn into(self) -> SvcSounds {
        match self {
            NetSvcMessageDataTypes::SvcSounds(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcSplitScreen> for NetSvcMessageDataTypes {
    fn into(self) -> SvcSplitScreen {
        match self {
            NetSvcMessageDataTypes::SvcSplitScreen(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcTempEntities> for NetSvcMessageDataTypes {
    fn into(self) -> SvcTempEntities {
        match self {
            NetSvcMessageDataTypes::SvcTempEntities(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcUpdateStringTable> for NetSvcMessageDataTypes {
    fn into(self) -> SvcUpdateStringTable {
        match self {
            NetSvcMessageDataTypes::SvcUpdateStringTable(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcUserMessage> for NetSvcMessageDataTypes {
    fn into(self) -> SvcUserMessage {
        match self {
            NetSvcMessageDataTypes::SvcUserMessage(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcVoiceData> for NetSvcMessageDataTypes {
    fn into(self) -> SvcVoiceData {
        match self {
            NetSvcMessageDataTypes::SvcVoiceData(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

impl Into<SvcVoiceInit> for NetSvcMessageDataTypes {
    fn into(self) -> SvcVoiceInit {
        match self {
            NetSvcMessageDataTypes::SvcVoiceInit(value) => value,
            _ => panic!("how are you even seeing this?"),
        }
    }
}

pub enum NetSvcMessageTypes {
    Unknown,
    NetNop,
    NetDisconnect,
    NetFile,
    NetSplitScreenUser,
    NetTick,
    NetStringCmd,
    NetSetConVar,
    NetSignonState,
    SvcPrint,
    SvcServerInfo,
    SvcSendTable,
    SvcClassInfo,
    SvcSetPause,
    SvcCreateStringTable,
    SvcUpdateStringTable,
    SvcVoiceInit,
    SvcVoiceData,
    SvcSounds,
    SvcSetView,
    SvcFixAngle,
    SvcCrosshairAngle,
    SvcBspDecal,
    SvcSplitScreen,
    SvcUserMessage,
    SvcEntityMessage,
    SvcGameEvent,
    SvcPacketEntities,
    SvcTempEntities,
    SvcPrefetch,
    SvcMenu,
    SvcGameEventList,
    SvcGetCvarValue,
    SvcCmdKeyValues,
    SvcPaintmapData,
}

impl NetSvcMessageTypes {
    pub fn from_int(value: i32) -> NetSvcMessageTypes {
        return match value {
            0 => NetSvcMessageTypes::NetNop,
            1 => NetSvcMessageTypes::NetDisconnect,
            2 => NetSvcMessageTypes::NetFile,
            3 => NetSvcMessageTypes::NetTick,
            4 => NetSvcMessageTypes::NetStringCmd,
            5 => NetSvcMessageTypes::NetSetConVar,
            6 => NetSvcMessageTypes::NetSignonState,
            7 => NetSvcMessageTypes::SvcPrint,
            8 => NetSvcMessageTypes::SvcServerInfo,
            9 => NetSvcMessageTypes::SvcSendTable,
            10 => NetSvcMessageTypes::SvcClassInfo,
            11 => NetSvcMessageTypes::SvcSetPause,
            12 => NetSvcMessageTypes::SvcCreateStringTable,
            13 => NetSvcMessageTypes::SvcUpdateStringTable,
            14 => NetSvcMessageTypes::SvcVoiceInit,
            15 => NetSvcMessageTypes::SvcVoiceData,
            16 => NetSvcMessageTypes::SvcPrint, // 16 is svc print in demo protocol 4, just adding it here so i dont have to later
            17 => NetSvcMessageTypes::SvcSounds,
            18 => NetSvcMessageTypes::SvcSetView,
            19 => NetSvcMessageTypes::SvcFixAngle,
            20 => NetSvcMessageTypes::SvcCrosshairAngle,
            21 => NetSvcMessageTypes::SvcBspDecal,
            22 => NetSvcMessageTypes::SvcSplitScreen,
            23 => NetSvcMessageTypes::SvcUserMessage,
            24 => NetSvcMessageTypes::SvcEntityMessage,
            25 => NetSvcMessageTypes::SvcGameEvent,
            26 => NetSvcMessageTypes::SvcPacketEntities,
            27 => NetSvcMessageTypes::SvcTempEntities,
            28 => NetSvcMessageTypes::SvcPrefetch,
            29 => NetSvcMessageTypes::SvcMenu,
            30 => NetSvcMessageTypes::SvcGameEventList,
            31 => NetSvcMessageTypes::SvcGetCvarValue,
            32 => NetSvcMessageTypes::SvcCmdKeyValues,
            33 => NetSvcMessageTypes::SvcPaintmapData,
            _ => NetSvcMessageTypes::Unknown,
        }
    }
}

pub struct NetSvcMessage {
    pub msg_type: NetSvcMessageTypes,
    pub data: NetSvcMessageDataTypes,
}

impl NetSvcMessage {
    pub fn new() -> Self {
        Self { msg_type: NetSvcMessageTypes::Unknown, data: NetSvcMessageDataTypes::Unknown }
    }
}

pub fn parse(reader: &mut BitReader, demo_data_mgr: &mut DataManager, size: i32) -> Vec<NetSvcMessage> {
    let mut messages: Vec<NetSvcMessage> = Vec::new();
    let start_index = reader.cur_bit_index;

    while ((start_index + size * 8) - reader.cur_bit_index) > 6 {
        let mut cur_message: NetSvcMessage = NetSvcMessage::new();

        let msg_type = reader.read_int(demo_data_mgr.net_svc_type_bits);

        cur_message.msg_type = nsmt::from_int(msg_type);

        match cur_message.msg_type {
            nsmt::Unknown => cur_message.data = nsmdt::Unknown,
            nsmt::NetNop => cur_message.data = nsmdt::NetNop(nt::NetNop),
            nsmt::NetDisconnect => cur_message.data = nsmdt::NetDisconnect(nt::NetDisconnect::parse(reader)),
            nsmt::NetFile => cur_message.data = nsmdt::NetFile(nt::NetFile::parse(reader)),
            nsmt::NetSetConVar => cur_message.data = nsmdt::NetSetConVar(nt::NetSetConVar::parse(reader)),
            nsmt::NetSignonState => cur_message.data = nsmdt::NetSignonState(nt::NetSignonState::parse(reader)),
            nsmt::NetSplitScreenUser => cur_message.data = nsmdt::NetSplitScreenUser(nt::NetSplitScreenUser::parse(reader)),
            nsmt::NetStringCmd => cur_message.data = nsmdt::NetStringCmd(nt::NetStringCmd::parse(reader)),
            nsmt::NetTick => cur_message.data = nsmdt::NetTick(nt::NetTick::parse(reader)),
            nsmt::SvcBspDecal => cur_message.data = nsmdt::SvcBspDecal(nt::SvcBspDecal::parse(reader)),
            nsmt::SvcClassInfo => cur_message.data = nsmdt::SvcClassInfo(nt::SvcClassInfo::parse(reader)),
            nsmt::SvcCmdKeyValues => cur_message.data = nsmdt::SvcCmdKeyValues(nt::SvcCmdKeyValues::parse(reader)),
            nsmt::SvcCreateStringTable => cur_message.data = nsmdt::SvcCreateStringTable(nt::SvcCreateStringTable::parse(reader, demo_data_mgr)),
            nsmt::SvcCrosshairAngle => cur_message.data = nsmdt::SvcCrosshairAngle(nt::SvcCrosshairAngle::parse(reader)),
            nsmt::SvcEntityMessage => cur_message.data = nsmdt::SvcEntityMessage(nt::SvcEntityMessage::parse(reader)),
            nsmt::SvcFixAngle => cur_message.data = nsmdt::SvcFixAngle(nt::SvcFixAngle::parse(reader)),
            nsmt::SvcGameEvent => cur_message.data = nsmdt::SvcGameEvent(nt::SvcGameEvent::parse(reader, &mut demo_data_mgr.game_event_list)),
            nsmt::SvcGameEventList => cur_message.data = nsmdt::SvcGameEventList(nt::SvcGameEventList::parse(reader, &mut demo_data_mgr.game_event_list)),
            nsmt::SvcGetCvarValue => cur_message.data = nsmdt::SvcGetCvarValue(nt::SvcGetCvarValue::parse(reader)),
            nsmt::SvcMenu => cur_message.data = nsmdt::SvcMenu(nt::SvcMenu::parse(reader)),
            nsmt::SvcPacketEntities => cur_message.data = nsmdt::SvcPacketEntities(nt::SvcPacketEntities::parse(reader)),
            nsmt::SvcPaintmapData => cur_message.data = nsmdt::SvcPaintmapData(nt::SvcPaintmapData::parse(reader)),
            nsmt::SvcPrefetch => cur_message.data = nsmdt::SvcPrefetch(nt::SvcPrefetch::parse(reader, &demo_data_mgr)),
            nsmt::SvcPrint => cur_message.data = nsmdt::SvcPrint(nt::SvcPrint::parse(reader)),
            nsmt::SvcSendTable => cur_message.data = nsmdt::SvcSendTable(nt::SvcSendTable::parse(reader)),
            nsmt::SvcServerInfo => cur_message.data = nsmdt::SvcServerInfo(nt::SvcServerInfo::parse(reader, demo_data_mgr)),
            nsmt::SvcSetPause => cur_message.data = nsmdt::SvcSetPause(nt::SvcSetPause::parse(reader)),
            nsmt::SvcSetView => cur_message.data = nsmdt::SvcSetView(nt::SvcSetView::parse(reader)),
            nsmt::SvcSounds => cur_message.data = nsmdt::SvcSounds(nt::SvcSounds::parse(reader)),
            nsmt::SvcSplitScreen => cur_message.data = nsmdt::SvcSplitScreen(nt::SvcSplitScreen::parse(reader)),
            nsmt::SvcTempEntities => cur_message.data = nsmdt::SvcTempEntities(nt::SvcTempEntities::parse(reader)),
            nsmt::SvcUpdateStringTable => cur_message.data = nsmdt::SvcUpdateStringTable(nt::SvcUpdateStringTable::parse(reader)),
            nsmt::SvcUserMessage => cur_message.data = nsmdt::SvcUserMessage(nt::SvcUserMessage::parse(reader, demo_data_mgr.user_message_list.clone())),
            nsmt::SvcVoiceData => cur_message.data = nsmdt::SvcVoiceData(nt::SvcVoiceData::parse(reader)),
            nsmt::SvcVoiceInit => cur_message.data = nsmdt::SvcVoiceInit(nt::SvcVoiceInit::parse(reader)),
        };

        messages.push(cur_message);
    }

    return messages;
}

#[allow(unused)]
pub fn write_msg_data_to_file(file: &mut File, messages: Vec<NetSvcMessage>, data_mgr: &DataManager) {
    for message in messages {
        match message.msg_type {
            nsmt::NetDisconnect => {
                let msg_data: nt::NetDisconnect = message.data.into();
                file.write_all("\n\tMessage: NetDisconnect".as_bytes());
                file.write_fmt(format_args!("\n\t\tText: {}\n", msg_data.text));
            },
            nsmt::NetFile => {
                let msg_data: nt::NetFile = message.data.into();
                file.write_all("\n\tMessage: NetFile".as_bytes());
                file.write_fmt(format_args!("\n\t\tTransfer ID: {}", msg_data.transfer_id));
                file.write_fmt(format_args!("\n\t\tFilename: {}", msg_data.filename));
                file.write_fmt(format_args!("\n\t\tFileRequested: {}", msg_data.file_requested));
            },
            nsmt::NetTick => {
                let msg_data: nt::NetTick = message.data.into();
                file.write_all("\n\tMessage: NetTick".as_bytes());
                file.write_fmt(format_args!("\n\t\tTick: {}", msg_data.tick));
                file.write_fmt(format_args!("\n\t\tHost Frame Time: {}", msg_data.host_frame_time as f32 / 1e5));
                file.write_fmt(format_args!("\n\t\tHost Frame Time Standard Deviation: {}", msg_data.host_frame_time_standard_deviation  as f32 / 1e5));
            },
            nsmt::NetNop => { file.write_all("\n\tMessage: NetNop".as_bytes()); },
            nsmt::NetStringCmd => {
                let msg_data: nt::NetStringCmd = message.data.into();
                file.write_all("\n\tMessage: NetStringCmd".as_bytes());
                file.write_fmt(format_args!("\n\t\tCommand: {}", msg_data.command));
            },
            nsmt::NetSetConVar => {
                let msg_data: nt::NetSetConVar = message.data.into();
                file.write_all("\n\tMessage: NetSetConVar".as_bytes());
                file.write_fmt(format_args!("\n\t\tLength: {}", msg_data.length));
                file.write_fmt(format_args!("\n\t\t{} convars:", msg_data.convars.len()));
                for i in 0..msg_data.convars.len() {
                    file.write_fmt(format_args!("\n\t\t{}: {}", msg_data.convars[i].convar_name, msg_data.convars[i].convar_value));
                }
            },
            nsmt::NetSignonState => {
                let msg_data: nt::NetSignonState = message.data.into();
                file.write_all("\n\tMessage: NetSignonState".as_bytes());
                file.write_fmt(format_args!("\n\t\tSignon State: {}", msg_data.signon_state));
                file.write_fmt(format_args!("\n\t\tSpawn Count: {}", msg_data.spawn_count));
            }
            nsmt::SvcServerInfo => {
                let msg_data: nt::SvcServerInfo = message.data.into();
                file.write_all("\n\tMessage: SvcServerInfo".as_bytes());
                file.write_fmt(format_args!("\n\t\tProtocol: {}", msg_data.protocol));
                file.write_fmt(format_args!("\n\t\tServerCount: {}", msg_data.server_count));
                file.write_fmt(format_args!("\n\t\tIs HLTV: {}", msg_data.is_hltv));
                file.write_fmt(format_args!("\n\t\tIs Dedicated: {}", msg_data.is_dedicated));
                file.write_fmt(format_args!("\n\t\tClient CRC: {}", msg_data.client_crc));
                file.write_fmt(format_args!("\n\t\tMax Classes: {}", msg_data.max_classes));
                if data_mgr.network_protocol == 24 {
                    let bytes = msg_data.map_md5.unwrap();
                    let mut hex_string: String = String::new();
                    for byte in bytes {
                        hex_string.push_str(&format_args!("{:02x}", byte).to_string())
                    }
                    file.write_fmt(format_args!("\n\t\tMap MD5: 0x{}", hex_string));
                } else {
                    file.write_fmt(format_args!("\n\t\tMap CRC: {}", msg_data.map_crc.map(|i| i.to_string()).unwrap_or_else(|| {"Null".to_string()})));
                }
                file.write_fmt(format_args!("\n\t\tPlayer Slot: {}", msg_data.player_slot));
                file.write_fmt(format_args!("\n\t\tMax Clients: {}", msg_data.max_clients));
                file.write_fmt(format_args!("\n\t\tPlatform: {}", msg_data.platform));
                file.write_fmt(format_args!("\n\t\tGame Directory: {}", msg_data.game_dir));
                file.write_fmt(format_args!("\n\t\tMap Name: {}", msg_data.map_name));
                file.write_fmt(format_args!("\n\t\tSkybox Name: {}", msg_data.sky_name));
                file.write_fmt(format_args!("\n\t\tHost Name: {}", msg_data.host_name));
            },
            nsmt::SvcSendTable => {
                let msg_data: nt::SvcSendTable = message.data.into();
                file.write_all("\n\tMessage: SvcSendTable".as_bytes());
                file.write_fmt(format_args!("\n\t\tNeeds Decoder: {}", msg_data.needs_decoder));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_fmt(format_args!("\n\t\tProps: {}", msg_data.props));
            },
            nsmt::NetSplitScreenUser => {
                let msg_data: nt::NetSplitScreenUser = message.data.into();
                file.write_all("\n\tMessage: NetSplitScreenUser".as_bytes());
                file.write_fmt(format_args!("\n\t\tUnknown: {}", msg_data.unknown));
            },
            nsmt::SvcBspDecal => {
                let msg_data: nt::SvcBspDecal = message.data.into();
                file.write_all("\n\tMessage: SvcBspDecal".as_bytes());
                file.write_fmt(format_args!("\n\t\tPos: {}, {}, {}",
                    msg_data.pos[0].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
                    msg_data.pos[1].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
                    msg_data.pos[2].map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
                file.write_fmt(format_args!("\n\t\tDecal Texture Index: {}", msg_data.decal_texture_index));
                file.write_fmt(format_args!("\n\t\tEntity Index: {}", msg_data.entity_index.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
                file.write_fmt(format_args!("\n\t\tModel Index: {}", msg_data.model_index.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
                file.write_fmt(format_args!("\n\t\tLow Priority: {}", msg_data.low_priority));
            },
            nsmt::SvcClassInfo => {
                let msg_data: nt::SvcClassInfo = message.data.into();
                file.write_all("\n\tMessage: SvcClassInfo".as_bytes());
                file.write_fmt(format_args!("\n\t\tCreate On Client: {}", msg_data.create_on_client));
                file.write_fmt(format_args!("\n\t\t{} server classes", msg_data.length));
                if !msg_data.create_on_client {
                    file.write(":".as_bytes());
                    for server_class in msg_data.server_classes {
                        file.write_fmt(format_args!("\n\t\t\t[{}] {} ({})", server_class.class_id, server_class.class_name, server_class.class_name));
                    }
                }
            },
            nsmt::SvcSetPause => {
                let msg_data: nt::SvcSetPause = message.data.into();
                file.write_all("\n\tMessage: SvcSetPause".as_bytes());
                file.write_fmt(format_args!("\n\t\tPaused: {}", msg_data.paused));
            },
            nsmt::SvcCreateStringTable => {
                let msg_data: nt::SvcCreateStringTable = message.data.into();
                file.write_all("\n\tMessage: SvcCreateStringTable".as_bytes());
                file.write_fmt(format_args!("\n\t\tName: {}", msg_data.name));
                file.write_fmt(format_args!("\n\t\tMax Entries: {}", msg_data.max_entries));
                file.write_fmt(format_args!("\n\t\tNum Entries: {}", msg_data.num_entries));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_fmt(format_args!("\n\t\tUser Data Fixed Size: {}", msg_data.user_data_fixed_size));
                file.write_fmt(format_args!("\n\t\tUser Data Size: {}", msg_data.user_data_size.map(|i| i.to_string()).unwrap_or_else(|| {"Null".to_string()})));
                file.write_fmt(format_args!("\n\t\tUser Data Size Bits: {}", msg_data.user_data_size_bits.map(|i| i.to_string()).unwrap_or_else(|| {"Null".to_string()})));
                file.write_fmt(format_args!("\n\t\tFlags: {}", msg_data.flags.map(|i| i.to_string()).unwrap_or_else(|| {"Null".to_string()})));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcUpdateStringTable => {
                let msg_data: nt::SvcUpdateStringTable = message.data.into();
                file.write_all("\n\tMessage: SvcUpdateStringTable".as_bytes());
                file.write_fmt(format_args!("\n\t\tTable ID: {}", msg_data.table_id));
                file.write_fmt(format_args!("\n\t\tNum Changed Entries: {}", msg_data.num_changed_entries));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcVoiceInit => {
                let msg_data: nt::SvcVoiceInit = message.data.into();
                file.write_all("\n\tMessage: SvcVoiceInit".as_bytes());
                file.write_fmt(format_args!("\n\t\tCodec: {}", msg_data.codec));
                file.write_fmt(format_args!("\n\t\tQuality: {}", msg_data.quality));
                file.write_fmt(format_args!("\n\t\tCodec: {}", msg_data.sample_rate.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
            },
            nsmt::SvcVoiceData => {
                let msg_data: nt::SvcVoiceData = message.data.into();
                file.write_all("\n\tMessage: SvcVoiceInit".as_bytes());
                file.write_fmt(format_args!("\n\t\tClient: {}", msg_data.client));
                file.write_fmt(format_args!("\n\t\tProximity: {}", msg_data.proximity));
                file.write_fmt(format_args!("\n\t\tLength: {}", msg_data.length));
                file.write_fmt(format_args!("\n\t\tAudible: {}", msg_data.audible[0])); // just keeping this at index 0 for now since im not parsing demos other than unpack yet
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcPrint => {
                let msg_data: nt::SvcPrint = message.data.into();
                file.write_all("\n\tMessage: SvcPrint".as_bytes());
                file.write_fmt(format_args!("\n\t\tMessage: {}", msg_data.message));
            },
            nsmt::SvcSounds => {
                let msg_data: nt::SvcSounds = message.data.into();
                file.write_all("\n\tMessage: SvcSounds".as_bytes());
                file.write_fmt(format_args!("\n\t\tReliable Sound: {}", msg_data.reliable_sound));
                file.write_fmt(format_args!("\n\t\tNum Sounds: {}", msg_data.num_sounds));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcSetView => {
                let msg_data: nt::SvcSetView = message.data.into();
                file.write_all("\n\tMessage: SvcSetView".as_bytes());
                file.write_fmt(format_args!("\n\t\tEntity Index: {}", msg_data.entity_index));
            },
            nsmt::SvcFixAngle => {
                let msg_data: nt::SvcFixAngle = message.data.into();
                file.write_all("\n\tMessage: SvcFixAngle".as_bytes());
                file.write_fmt(format_args!("\n\t\tRelative: {}", msg_data.relative));
                file.write_fmt(format_args!("\n\t\tAngle: {:.3} {:.3} {:.3}", msg_data.angle[0], msg_data.angle[1], msg_data.angle[2]));
            },
            nsmt::SvcCrosshairAngle => {
                let msg_data: nt::SvcCrosshairAngle = message.data.into();
                file.write_all("\n\tMessage: SvcCrosshairAngle".as_bytes());
                file.write_fmt(format_args!("\n\t\tAngle: {} {} {}", msg_data.angle[0], msg_data.angle[1], msg_data.angle[2]));
            },
            nsmt::SvcUserMessage => {
                let msg_data: nt::SvcUserMessage = message.data.into();
                file.write_all("\n\tMessage: SvcUserMessage".as_bytes());
                file.write_fmt(format_args!("\n\t\tMessage Type: {:?}", msg_data.data.msg_type));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_all("\n\t\tData:".as_bytes());
                write_usermsg_data_to_file(msg_data.data, file);
            },
            nsmt::SvcEntityMessage => {
                let msg_data: nt::SvcEntityMessage = message.data.into();
                file.write_all("\n\tMessage: SvcEntityMessage".as_bytes());
                file.write_fmt(format_args!("\n\t\tEntity Index: {}", msg_data.entity_index));
                file.write_fmt(format_args!("\n\t\tClass ID: {}", msg_data.class_id));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
            },
            nsmt::SvcGameEvent => {
                let msg_data: nt::SvcGameEvent = message.data.into();
                file.write_all("\n\tMessage: SvcGameEvent".as_bytes());
                file.write_fmt(format_args!("\n\t\tLength: {}", msg_data.length));

                let event = msg_data.data;
                file.write_fmt(format_args!("\n\t\t{} ({})", event.descriptor.name, event.descriptor.event_id));
                for key in event.keys {
                    file.write_fmt(format_args!("\n\t\t\t{}: {}", key.0, key.1.to_string()));
                }
            },
            nsmt::SvcPacketEntities => {
                let msg_data: nt::SvcPacketEntities = message.data.into();
                file.write_all("\n\tMessage: SvcPacketEntities".as_bytes());
                file.write_fmt(format_args!("\n\t\tMax Entries: {}", msg_data.max_entries));
                file.write_fmt(format_args!("\n\t\tIs Delta: {}", msg_data.is_delta));
                file.write_fmt(format_args!("\n\t\tDelta From: {}", msg_data.delta_from.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
                file.write_fmt(format_args!("\n\t\tBaseline: {}", msg_data.base_line));
                file.write_fmt(format_args!("\n\t\tUpdated Entries: {}", msg_data.updated_entries));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_fmt(format_args!("\n\t\tUpdata Baseline: {}", msg_data.update_baseline));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcTempEntities => {
                let msg_data: nt::SvcTempEntities = message.data.into();
                file.write_all("\n\tMessage: SvcTempEntities".as_bytes());
                file.write_fmt(format_args!("\n\t\tNum Entries: {}", msg_data.num_entries));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcPrefetch => {
                let msg_data: nt::SvcPrefetch = message.data.into();
                file.write_all("\n\tMessage: SvcPrefetch".as_bytes());
                file.write_fmt(format_args!("\n\t\tSound Index: {}", msg_data.sound_index));
            },
            nsmt::SvcMenu => {
                let msg_data: nt::SvcMenu = message.data.into();
                file.write_all("\n\tMessage: SvcMenu".as_bytes());
                file.write_fmt(format_args!("\n\t\tMenu Type: {}", msg_data.length));
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcGameEventList => {
                let msg_data: &GameEventList = &data_mgr.game_event_list;
                file.write_all("\n\tMessage: SvcGameEventList".as_bytes());
                file.write_fmt(format_args!("\n\t\tLength: {}", msg_data.length));
                file.write_fmt(format_args!("\n\t\t{} events:", msg_data.events));
                
                for event in &msg_data.data {
                    file.write_fmt(format_args!("\n\t\t\t{}: {}", event.descriptor.event_id, event.descriptor.name));
                    file.write_all("\n\t\t\t\tKeys: [".as_bytes());
                    let mut keys_str: String = "".to_string();
                    for (name, value_type) in &event.descriptor.keys {
                        keys_str.push_str(&format_args!("{} {}", match value_type {
                            1 => "string",
                            2 => "float",
                            3 => "int32",
                            4 => "int16",
                            5 => "int8",
                            6 => "bool",
                            7 => "uint64",
                            _ => "unknown type"
                        }, name).to_string());
                        keys_str.push_str(", ");
                    }
                    keys_str = if keys_str != "".to_string() { keys_str[..keys_str.len()-2].to_string() } else { "".to_string() };
                    file.write_fmt(format_args!("{}]", keys_str));
                }
            },
            nsmt::SvcGetCvarValue => {
                let msg_data: nt::SvcGetCvarValue = message.data.into();
                file.write_all("\n\tMessage: SvcGetCvarValue".as_bytes());
                file.write_fmt(format_args!("\n\t\tCookie: {}", msg_data.cookie));
                file.write_fmt(format_args!("\n\t\tCvar Name: {}", msg_data.cvar_name));
            },
            nsmt::SvcCmdKeyValues => {
                let msg_data: nt::SvcCmdKeyValues = message.data.into(); 
                file.write_fmt(format_args!("\n\t\tLength (bytes): {}", msg_data.length));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcPaintmapData => {
                let msg_data: nt::SvcPaintmapData = message.data.into();
                file.write_all("\n\tMessage: SvcPaintmapData".as_bytes());
                file.write_fmt(format_args!("\n\t\tLength (bits): {}", msg_data.length));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            nsmt::SvcSplitScreen => {
                let msg_data: nt::SvcSplitScreen = message.data.into();
                file.write_all("\n\tMessage: SvcSplitScreen".as_bytes());
                file.write_fmt(format_args!("\n\tType: {}", msg_data.s_type));
                file.write_fmt(format_args!("\n\t\tLength (bytes): {}", msg_data.length));
                file.write_all("\n\t\tNO MORE DATA AVAILABLE (yet)".as_bytes());
            },
            _ => { file.write_all("\n\tMessage: type unknown :(".as_bytes()); }
        }
    }
    file.write_all("\n".as_bytes());
}