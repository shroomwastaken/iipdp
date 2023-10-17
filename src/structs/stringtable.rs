use std::fs::File;
use std::io::Write;
use bitflags::bitflags;
use crate::bitreader::BitReader;
use crate::structs::packet_data_types::StringTables;

// all of this information is from UntitledParser

pub struct StringTable {
    pub name: String,
    pub entry_count: i32,
    pub class_count: i32,
    pub table_entries: Vec<StringTableEntry>,
    pub classes: Vec<StringTableClass>,
}

impl StringTable {
    pub fn parse(reader: &mut BitReader) -> Self {
        let name = reader.read_ascii_string_nulled();
        let entry_count = reader.read_int(16);
        let mut class_count = 0;
        let mut table_entries: Vec<StringTableEntry> = Vec::new();
        let mut classes: Vec<StringTableClass> = Vec::new();
        if entry_count > 0 {
            for _ in 0..entry_count {
                let mut entry = StringTableEntry::new();
                entry.parse(reader, name.clone());
                table_entries.push(entry);
            }
        }
        if reader.read_bool() {
            class_count = reader.read_int(16);
            for _ in 0..class_count {
                let mut class = StringTableClass::new();
                class.parse(reader);
                classes.push(class);
            }
        }

        Self { name: name, entry_count: entry_count, class_count: class_count, table_entries: table_entries, classes: classes }
    }
}

pub struct StringTableEntry {
    pub name: String,
    pub entry_data: StringTableEntryDataTypes
}

impl StringTableEntry {
    pub fn new() -> Self {
        Self { name: "".to_string(), entry_data: StringTableEntryDataTypes::None }
    }

    pub fn parse(&mut self, reader: &mut BitReader, table_name: String) {
        self.name = reader.read_ascii_string_nulled();
        if reader.read_bool() {
            let length = reader.read_int(16);
            self.entry_data = self.parse_entry_data(&mut reader.split_and_skip(length * 8), table_name, length);
        }
    }

    pub fn parse_entry_data(&self, reader: &mut BitReader, table_name: String, length: i32) -> StringTableEntryDataTypes {
        match table_name.as_str() {
            "userinfo" => StringTableEntryDataTypes::PlayerInfo(PlayerInfo::parse(reader)),
            "server_query_info" => StringTableEntryDataTypes::QueryPort(QueryPort::parse(reader)),
            "GameRulesCreation" => StringTableEntryDataTypes::StringEntryData(StringEntryData::parse(reader)),
            "InfoPanel" => StringTableEntryDataTypes::StringEntryData(StringEntryData::parse(reader)),
            "lightstyles" => StringTableEntryDataTypes::LightStyle(LightStyle::parse(reader)),
            "modelprecache" => StringTableEntryDataTypes::PrecacheData(PrecacheData::parse(reader)),
            "soundprecache" => StringTableEntryDataTypes::PrecacheData(PrecacheData::parse(reader)),
            "genericprecache" => StringTableEntryDataTypes::PrecacheData(PrecacheData::parse(reader)),
            "decalprecache" => StringTableEntryDataTypes::PrecacheData(PrecacheData::parse(reader)),
            _ => { reader.skip(length * 8); StringTableEntryDataTypes::Unknown }
        }
    }
}

// all of the currently implemented data types
pub enum StringTableEntryDataTypes {
    None,
    Unknown,
    PlayerInfo(PlayerInfo),
    QueryPort(QueryPort),
    StringEntryData(StringEntryData),
    LightStyle(LightStyle),
    PrecacheData(PrecacheData),
}

pub struct StringTableClass {
    pub name: String,
    pub length: i32,
    pub data: String,
}

impl StringTableClass {
    pub fn new() -> Self {
        Self { name: "".to_string(), length: 0, data: "".to_string() }
    }

    pub fn parse(&mut self, reader: &mut BitReader) {
        self.name = reader.read_ascii_string_nulled();
        if reader.read_bool() {
            self.length = reader.read_int(16);
            self.data = reader.read_ascii_string(self.length * 8);
        }
    }
}

// entry data types

pub struct PlayerInfo {
    // steam id stuff only exists on demo protocol 4 so im not gonna bother yet
    pub name: String, // scoreboard info
    pub user_id: i32, // local server user id, unique while the server is running
    pub guid: String, // global unique player identifier
    pub friends_id: i32, // friends identification number
    pub friends_name: String,
    pub fake_player: bool, // true if player is a bot controlled by game.dll
    pub is_hltv: bool, // true if player is hltv proxy
    pub custom_files: Vec<i32>, // custom files crc for this player
    pub files_downloaded: i32, // this counter increases each time the server downloaded a new file
}

impl PlayerInfo {
    // this is actually a player_info_t so we ahve to stay byte-aligned
    pub fn parse(reader: &mut BitReader) -> Self {
        let name = reader.read_ascii_string(32 * 8);
        let user_id = reader.read_int(32);
        let guid = reader.read_ascii_string(33 * 8);
        reader.skip(24);
        let friends_id = reader.read_int(32);
        let friends_name = reader.read_ascii_string(32 * 8);
        let fake_player = reader.read_int(8) != 0;
        let is_hltv = reader.read_int(8) != 0;
        reader.skip(16);
        let custom_files: Vec<i32> = vec![reader.read_int(23), reader.read_int(23), reader.read_int(23), reader.read_int(23)];
        let files_donwloaded = reader.read_int(8);
        reader.skip(24);
        Self {
            name: name,
            user_id: user_id,
            guid: guid,
            friends_id: friends_id,
            friends_name: friends_name,
            fake_player: fake_player,
            is_hltv: is_hltv,
            custom_files: custom_files,
            files_downloaded: files_donwloaded
        }
    }
}

pub struct QueryPort {
    pub port: i32,
}

impl QueryPort {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { port: reader.read_int(32) }
    }
}

pub struct InstanceBaseline; // not until i do datatable stuff

pub struct StringEntryData {
    pub str: String,
}

impl StringEntryData {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { str: reader.read_ascii_string_nulled() }
    }
}

pub struct LightStyle {
    pub values: Vec<u8>,
}

impl LightStyle {
    pub fn parse(reader: &mut BitReader) -> Self {
        let str = reader.read_ascii_string_nulled();
        let values: Vec<u8> = str.chars().map(|c| ((c as u32 - 'a' as u32) * 22) as u8).collect::<_>(); // idk
        Self { values: values }
    }
}

pub struct PrecacheData {
    pub flags: PrecacheFlags,
}

impl PrecacheData {
    pub fn parse(reader: &mut BitReader) -> Self {
        Self { flags: PrecacheFlags::from_bits_truncate(reader.read_int(2)) }
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct PrecacheFlags: i32 {
        const None = 0;
        const FatalIfMissing = 1;
        const Preload = 1 << 1;
    }
}

#[allow(unused)]
pub fn write_stringtables_data_to_file(file: &mut File, data: StringTables) {
    file.write_fmt(format_args!("\tData Size (bytes): {}\n", data.size));
    file.write_fmt(format_args!("\tTable Count: {}", data.table_count));
}