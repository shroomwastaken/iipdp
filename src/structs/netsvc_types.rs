use crate::bitreader::BitReader;
use crate::structs::utils;

// contains no data
pub struct NetNop;

pub struct NetDisconnect {
    pub text: String,
}

impl NetDisconnect {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { text: reader.read_ascii_string_nulled(), }
    }
}

pub struct NetFile {
    pub transfer_id: i32,
    pub filename: String,
    pub file_requested: bool,
}

impl NetFile {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            transfer_id: reader.read_int(32),
            filename: reader.read_ascii_string_nulled(),
            file_requested: reader.read_bool(),
        }
    }
}

pub struct NetSplitScreenUser {
    pub unknown: bool,
}

impl NetSplitScreenUser {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            unknown: reader.read_bool(),
        }
    }
}

pub struct NetTick {
    pub tick: i32,
    pub host_frame_time: i32,
    pub host_frame_time_standard_deviation: i32,
}

impl NetTick {
    pub fn parse(reader: &mut BitReader) -> Self {
        println!("here");
        Self {
            tick: reader.read_int(32),
            host_frame_time: reader.read_int(16),
            host_frame_time_standard_deviation: reader.read_int(16),
        }
    }
}

pub struct NetStringCmd {
    pub command: String,
}

impl NetStringCmd {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { command: reader.read_ascii_string_nulled(), }
    }
}

pub struct NetSetConVar {
    pub length: i32,
    pub convars: Vec<utils::ConVar>,
}

impl NetSetConVar {
    pub fn parse(reader: &mut BitReader) -> Self {
        let length: i32 = reader.read_int(8);

        let mut convars: Vec<utils::ConVar> = Vec::new();
        
        for _ in 0..length {
            let mut cur_convar: utils::ConVar = utils::ConVar::new();

            cur_convar.convar_name = reader.read_ascii_string_nulled();
            cur_convar.convar_value = reader.read_ascii_string_nulled();

            convars.push(cur_convar);
        }

        Self {
            length: length,
            convars: convars, 
        }
    }
}

pub struct NetSignonState {
    pub signon_state: i32,
    pub spawn_count: i32,
    // theres more for demoprotocol 4 but i wont do it yet
}

impl NetSignonState {
    pub fn parse(reader: &mut BitReader) ->  Self {
        Self {
            signon_state: reader.read_int(8),
            spawn_count: reader.read_int(32),
        }
    }
}

pub struct SvcServerInfo {
    pub protocol: i32,
    pub server_count: i32,
    pub is_hltv: bool,
    pub is_dedicated: bool,
    pub client_crc: i32,
    pub max_classes: i32,
    pub tick_interval: f32,
    pub map_crc: i32,
    pub player_slot: i32,
    pub max_clients: i32,
    pub platform: char,
    pub game_dir: String,
    pub map_name: String,
    pub sky_name: String,
    pub host_name: String,
}

impl SvcServerInfo {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            protocol: reader.read_int(16),
            server_count: reader.read_int(32),
            is_hltv: reader.read_bool(),
            is_dedicated: reader.read_bool(),
            client_crc: reader.read_int(32),
            max_classes: reader.read_int(16),
            map_crc: reader.read_int(32),
            player_slot: reader.read_int(8),
            max_clients: reader.read_int(8),
            tick_interval: reader.read_float(32),
            platform: reader.read_ascii_string(8).chars().next().unwrap(),
            game_dir: reader.read_ascii_string_nulled(),
            map_name: reader.read_ascii_string_nulled(),
            sky_name: reader.read_ascii_string_nulled(),
            host_name: reader.read_ascii_string_nulled(),
        }
    }
}

pub struct SvcSendTable {
    pub needs_decoder: bool,
    pub length: i32,
    pub props: i32,
}

impl SvcSendTable {
    pub fn parse(reader: &mut BitReader) -> Self {
        let needs_decoder = reader.read_bool();
        let length = reader.read_int(8);
        Self {
            needs_decoder: needs_decoder,
            length: length,
            props: reader.read_int(length),
        }
    }
}

pub struct SvcClassInfo {
    pub length: i32,
    pub create_on_client: bool,
    pub server_classes: Vec<utils::ServerClass>,
}

impl SvcClassInfo {
    pub fn parse(reader: &mut BitReader) -> Self {
        let length = reader.read_int(16);
        let create_on_client = reader.read_bool();
        let mut server_classes: Vec<utils::ServerClass> = Vec::new();

        if !create_on_client {
            for _ in 0..length {

                let mut cur_server_class: utils::ServerClass = utils::ServerClass::new();

                cur_server_class.class_id = reader.read_int((length as f32).log2() as i32 + 1);
                cur_server_class.class_name = reader.read_ascii_string_nulled();
                cur_server_class.data_table_name = reader.read_ascii_string_nulled();

                server_classes.push(cur_server_class);
            }
        }

        Self {
            length: length,
            create_on_client: create_on_client,
            server_classes: server_classes,
        }
    }
}

pub struct SvcSetPause {
    pub paused: bool,
}

impl SvcSetPause {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { paused: reader.read_bool() }
    }
}

pub struct SvcCreateStringTable {
    pub name: String,
    pub max_entries: i32,
    pub num_entries: i32,
    pub length: i32,
    pub user_data_fixed_size: bool,
    pub user_data_size: Option<i32>,
    pub user_data_size_bits: Option<i32>,
    pub flags: i32,
    pub string_data: utils::StringTable, // placeholder for now
}

impl SvcCreateStringTable {
    pub fn parse(reader: &mut BitReader) -> Self {
        let name = reader.read_ascii_string_nulled();
        let max_entries = reader.read_int(16);
        let num_entries = reader.read_int(((max_entries as f32).log2() as i32) + 1);
        let length = reader.read_int(20);
        let user_data_fixed_size = reader.read_bool();

        let mut user_data_size: Option<i32> = None;
        let mut user_data_size_bits: Option<i32> = None;

        if user_data_fixed_size {
            user_data_size = Some(reader.read_int(12));
            user_data_size_bits = Some(reader.read_int(4));
        }
        
        // this is 2 bits on demo protocol 4 !!! (will do later)
        let flags = reader.read_int(1);

        let string_data: utils::StringTable = utils::StringTable::new(); // placeholder
        reader.skip(length as i32); // skip bits for now

        Self {
            name: name,
            max_entries: max_entries,
            num_entries: num_entries,
            length: length,
            user_data_fixed_size: user_data_fixed_size,
            user_data_size: user_data_size,
            user_data_size_bits: user_data_size_bits,
            flags: flags,
            string_data: string_data,
        }        
    }
}

pub struct SvcUpdateStringTable {
    pub table_id: i32,
    pub num_changed_entries: i32,
    pub length: i32,
    pub data: utils::StringTable, // also just a placeholder
}

impl SvcUpdateStringTable {
    pub fn parse(reader: &mut BitReader) -> Self {
        let table_id = reader.read_int(5);
        let num_changed_entries = reader.read_int_if_exists(16).unwrap_or_else(|| {1});

        let length = reader.read_int(20);
        let data = utils::StringTable::new(); // nothin
        reader.skip(length as i32);
        Self {
            table_id: table_id,
            num_changed_entries: num_changed_entries,
            length: length,
            data: data,
        }
    }
}

pub struct SvcVoiceInit {
    pub codec: String,
    pub quality: i32,
    pub sample_rate: Option<i32>,
}

impl SvcVoiceInit {
    pub fn parse(reader: &mut BitReader) -> Self {
        let codec = reader.read_ascii_string_nulled();
        let quality = reader.read_int(8);

        let mut sample_rate: Option<i32> = None;

        if quality == 255 {
            sample_rate = Some(reader.read_int(32));
        }

        Self { codec: codec, quality: quality, sample_rate: sample_rate }
    }
}

pub struct SvcVoiceData {
    pub client: i32,
    pub proximity: i32,
    pub length: i32,
    pub audible: Vec<bool>,
    pub data: utils::VoiceData, // placeholder
}

impl SvcVoiceData {
    pub fn parse(reader: &mut BitReader) -> Self {
        let client = reader.read_int(8);
        let proximity = reader.read_int(8);
        let length = reader.read_int(16);

        // this will only have one element for now as im not parsing 2-player games
        let audible: Vec<bool> = vec![reader.read_bool()];

        // placeholder
        let data: utils::VoiceData = utils::VoiceData::new();

        // skip bits
        reader.skip(length as i32);

        Self {
            client: client,
            proximity: proximity,
            length: length,
            audible: audible,
            data: data,
        }
    }
}

pub struct SvcPrint {
    pub message: String,
}

impl SvcPrint {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { message: reader.read_ascii_string_nulled() }
    }
}

pub struct SvcSounds {
    pub reliable_sound: bool,
    pub num_sounds: i32,
    pub length: i32,
    pub data: Vec<utils::SoundInfo>,
}

impl SvcSounds {
    pub fn parse(reader: &mut BitReader) -> Self {
        let reliable_sounds = reader.read_bool();

        let num_sounds: i32;
        let length: i32;

        if reliable_sounds {
            num_sounds = 1;
            length = reader.read_int(8);
        } else {
            num_sounds = reader.read_int(8);
            length = reader.read_int(16);
        }

        let data: Vec<utils::SoundInfo> = Vec::new(); // placeholder
        reader.skip(length as i32); // skip

        Self { reliable_sound: reliable_sounds, num_sounds: num_sounds, length: length, data: data }
    }
}

pub struct SvcSetView {
    pub entity_index: i32,
}

impl SvcSetView {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { entity_index: reader.read_int(11) }
    }
}

pub struct SvcFixAngle {
    pub relative: bool,
    pub angle: Vec<f32>,
}

impl SvcFixAngle {
    pub fn parse(reader: &mut BitReader) -> Self {
        let relative = reader.read_bool();

        let mut angle_vec: Vec<f32> = Vec::new();

        for _ in 0..3 {
            angle_vec.push(reader.read_float(16));
        }

        Self { relative: relative, angle: angle_vec }
    }
}

pub struct SvcCrosshairAngle {
    pub angle: Vec<f32>,
}

impl SvcCrosshairAngle {
    pub fn parse(reader: &mut BitReader) -> Self {
        let mut angle_vec: Vec<f32> = Vec::new();

        for _ in 0..2 {
            angle_vec.push(reader.read_float(16));
        }

        Self { angle: angle_vec }
    }
}

pub struct SvcBspDecal; // no idea how to parse yet, havent seen in demos so im just gonna not touch it yet

pub struct SvcSplitScreen {
    pub s_type: i32,
    pub length: i32,
    pub data: utils::SplitScreenData,
}

impl SvcSplitScreen {
    pub fn parse(reader: &mut BitReader) -> Self {
        let s_type = reader.read_int(1);
        let length = reader.read_int(11);
        
        let data: utils::SplitScreenData = utils::SplitScreenData::new(); // placeholder
        reader.skip(length as i32); // skip

        Self { s_type: s_type, length: length, data: data }
    }
}

pub struct SvcUserMessage {
    pub msg_type: i32,
    pub length: i32,
    pub data: utils::UserMessageData,
}

impl SvcUserMessage {
    pub fn parse(reader: &mut BitReader) -> Self {
        let msg_type = reader.read_int(8);
        let length = reader.read_int(11);
        
        let data: utils::UserMessageData = utils::UserMessageData::new(); // placeholder
        reader.skip(length as i32); //skip

        Self { msg_type: msg_type, length: length, data: data }
    }
}

pub struct SvcEntityMessage {
    pub entity_index: i32,
    pub class_id: i32,
    pub length: i32,
    pub data: utils::EntityMessageData,
}

impl SvcEntityMessage {
    pub fn parse(reader: &mut BitReader) -> Self {
        let entity_index = reader.read_int(11);
        let class_id = reader.read_int(9);
        let length = reader.read_int(11);

        let data: utils::EntityMessageData = utils::EntityMessageData::new(); // placeholder
        reader.skip(length as i32); // skip

        Self { entity_index: entity_index, class_id: class_id, length: length, data: data }
    }
}

pub struct SvcGameEvent {
    pub length: i32,
    pub data: Vec<utils::GameEvent>,
}

impl SvcGameEvent {
    pub fn parse(reader: &mut BitReader, game_event_list: &mut utils::GameEventList) -> Self {
        let length = reader.read_int(11);
        let mut data: Vec<utils::GameEvent> = Vec::new();

        let event_id = reader.read_int(9);

        for mut event in &mut game_event_list.data {
            if event.descriptor.event_id == event_id {
                for (name, value_type) in &event.descriptor.keys {
                    match value_type {
                        1 => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::String(reader.read_ascii_string_nulled())),
                        2 => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::Float(reader.read_float(32))),
                        3 => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::Int32(reader.read_int(32))),
                        4 => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::Int16(reader.read_int(16))),
                        5 => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::Int8(reader.read_int(8))),
                        6 => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::Boolean(reader.read_bool())),
                        7 => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::UInt64(reader.read_uint_64())),
                        i32::MIN..=0_i32 | 8_i32..=i32::MAX => event.keys.insert(name.to_string(), utils::GameEventKeyTypes::None),
                    };
                }
                data.push(event.clone());
            }
        }

        Self { length: length, data: data }
    }
}

pub struct SvcPacketEntities {
    pub max_entries: i32,
    pub is_delta: bool,
    pub delta_from: Option<i32>,
    pub base_line: bool,
    pub updated_entries: i32,
    pub length: i32,
    pub update_baseline: bool,
    pub data: utils::PacketEntitiesData,
}

impl SvcPacketEntities {
    pub fn parse(reader: &mut BitReader) -> Self {
        let max_entries = reader.read_int(11);
        let is_delta = reader.read_bool();
        let mut delta_from: Option<i32> = None;
        if is_delta {
            delta_from = Some(reader.read_int(32));
        }
        let base_line = reader.read_bool();
        let updated_entries = reader.read_int(11);
        let length = reader.read_int(20);
        let update_baseline = reader.read_bool();
        let data:utils::PacketEntitiesData = utils::PacketEntitiesData::new(); // placeholder
        reader.skip(length as i32); // skip
        Self {
            max_entries: max_entries,
            is_delta: is_delta,
            delta_from: delta_from,
            base_line: base_line,
            updated_entries: updated_entries,
            length: length,
            update_baseline: update_baseline,
            data: data,
        }

    }
}

pub struct SvcTempEntities {
    pub num_entries: i32,
    pub length: i32,
    pub data: utils::TempEntitiesData,
}

impl SvcTempEntities {
    pub fn parse(reader: &mut BitReader) -> Self {
        let num_entires = reader.read_int(8);
        let length = reader.read_int(17);
        let data: utils::TempEntitiesData = utils::TempEntitiesData::new();
        reader.skip(length as i32);
        Self { num_entries: num_entires, length: length, data: data }
    }
}

pub struct SvcPrefetch {
    pub sound_index: i32,
}

impl SvcPrefetch {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { sound_index: reader.read_int(13) }
    }
}

pub struct SvcMenu {
    pub menu_type: i32,
    pub length: i32,
    pub data: utils::MenuData,
}

impl SvcMenu {
    pub fn parse(reader: &mut BitReader) -> Self {
        let menu_type = reader.read_int(16);
        let length = reader.read_int(32);
        let data: utils::MenuData = utils::MenuData::new();
        Self { menu_type: menu_type, length: length, data: data }
    }
}

pub struct SvcGameEventList {
    pub events: i32,
    pub length: i32,
    pub data: Vec<utils::GameEventDescriptor>,
}

impl SvcGameEventList {
    pub fn parse(reader: &mut BitReader) -> Self {
        let events = reader.read_int(9);
        let length = reader.read_int(20);
        let data: Vec<utils::GameEventDescriptor> = Vec::new();

        for _ in 0..events {
            let cur_event_desc = utils::GameEventDescriptor::parse(reader);
        }

        Self { events: events, length: length, data: data }
    }
}

pub struct SvcGetCvarValue {
    pub cookie: String,
    pub cvar_name: String,
}

impl SvcGetCvarValue {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self {
            cookie: reader.read_ascii_string(32),
            cvar_name: reader.read_ascii_string_nulled(),
        }
    }
}

pub struct SvcCmdKeyValues {
    pub length: i32,
    pub data: utils::CmdKeyValuesData,
}

impl SvcCmdKeyValues {
    pub fn parse(reader: &mut BitReader) -> Self {
        let length = reader.read_int(32);
        let data: utils::CmdKeyValuesData = utils::CmdKeyValuesData::new();
        reader.skip((length*8) as i32);

        Self { length: length, data: data }
    }
}

pub struct SvcPaintmapData {
    pub length: i32,
    pub data: utils::PaintmapData
}

impl SvcPaintmapData {
    pub fn parse(reader: &mut BitReader) -> Self {
        let length = reader.read_int(32);
        let data: utils::PaintmapData = utils::PaintmapData::new();
        reader.skip(length as i32);

        Self { length: length, data: data }
    }
}