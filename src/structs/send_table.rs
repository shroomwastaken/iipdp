use crate::bitreader::BitReader;
use crate::enum_primitive::enum_from_primitive;
use crate::structs::data_manager::DataManager;
use std::fs::File;
use std::io::Write;
use crate::structs::utils::bitflags_to_string;

#[derive(Debug, Clone)]
pub struct SendTable {
    pub needs_decoder: bool,
    pub name: String,
    pub prop_count: i32,
    pub prop_list: Vec<SendTableProp>,
}

impl SendTable {
    pub fn parse(reader: &mut BitReader, data_mgr: &DataManager) -> Self {
        let needs_decoder = reader.read_bool();
        let name = reader.read_ascii_string_nulled();
        let prop_count = reader.read_int(10);
        let mut prop_list: Vec<SendTableProp> = Vec::new();
        for _ in 0..prop_count {
            let cur_prop = SendTableProp::parse(reader, data_mgr);
            prop_list.push(cur_prop);
        }

        Self {
            needs_decoder: needs_decoder,
            name: name,
            prop_count: prop_count,
            prop_list: prop_list,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SendTableProp {
    pub send_prop_type: SendPropType,
    pub name: String,
    pub flags: PropFlag,
    pub exclude_dt_name: Option<String>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub num_bits: Option<i32>,
    pub num_elements: Option<i32>,
    pub float_parse_type: FloatParseType
}

impl SendTableProp {
    pub fn parse(reader: &mut BitReader, data_mgr: &DataManager) -> Self {
        let send_prop_type: SendPropType = data_mgr.send_prop_type_list[reader.read_int(5) as usize].clone();
        let name = reader.read_ascii_string_nulled();
        let flags = PropFlag::from_bits_truncate(reader.read_int(16)); // hardcoded bc im not doing anything other than p1 for now
        
        let mut exclude_dt_name: Option<String> = None;
        let mut low_value: Option<f32> = None;
        let mut high_value: Option<f32> = None;
        let mut num_bits: Option<i32> = None;
        let mut num_elements: Option<i32> = None;

        if send_prop_type == SendPropType::DataTable || flags.contains(PropFlag::Exclude) {
            exclude_dt_name = Some(reader.read_ascii_string_nulled());
        } else {
            match send_prop_type {
                SendPropType::String | SendPropType::Int | SendPropType::Float | SendPropType::Vector3 | SendPropType::Vector2 => {
                    low_value = Some(reader.read_float(32));
                    high_value = Some(reader.read_float(32));
                    num_bits = Some(reader.read_int(data_mgr.send_prop_amount_of_bits_to_get_num_bits));
                },
                SendPropType::Array => {
                    num_elements = Some(reader.read_int(10));
                },
                _ => panic!("lmao")
            }
        }

        Self {
            send_prop_type: send_prop_type,
            name: name,
            flags: flags,
            exclude_dt_name: exclude_dt_name,
            low_value: low_value,
            high_value: high_value,
            num_bits: num_bits,
            num_elements: num_elements,
            float_parse_type: FloatParseType::None,
        }
    }
}

enum_from_primitive! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum SendPropType {
        Int,
        Float,
        Vector3,
        Vector2,
        String,
        Array,
        DataTable,
    }
}

impl Into<&str> for SendPropType {
    fn into(self) -> &'static str {
        match self {
            Self::Int => "Int",
            Self::Float => "Float",
            Self::Vector2 => "Vector2",
            Self::Vector3 => "Vector3",
            Self::String => "String",
            Self::Array => "Array",
            Self::DataTable => "DataTable",
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Clone)]
    pub struct PropFlag : i32 {
        const Unsigned = 1;
        const Coord = 1 << 1;
        const NoScale = 1 << 2;
        const RoundDown = 1 << 3;
        const RoundUp = 1 << 4;
        const Normal = 1 << 5;
        const Exclude = 1 << 6;
        const Xyze = 1 << 7;
        const InsideArray = 1 << 8;
        const ProxyAlwaysYes = 1 << 9;
        const ChangesOften = 1 << 10;
        const IsVectorElem = 1 << 11;
        const Collapsible = 1 << 12;
        const CoordMp = 1 << 13;
        const CoordMpLp = 1 << 14; // low precision
        const CoordMpInt = 1 << 15;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FloatParseType {
    None, // i dont want to use option<>
    Standard,
    Coord,
    BitCoordMp,
    BitCoordMpLp,
    BitCoordMpInt,
    NoScale,
    Normal,
    BitCellChord,
    BitCellChordLp,
    BitCellChordInt,
}

// this function will make you wonder why i chose programming instead of something else as a hobby
#[allow(unused)]
pub fn write_send_table_data_to_file(file: &mut File, table: SendTable) {
    file.write_fmt(format_args!("\n\t\t{}, Needs Decoder: {}, Prop Count: {}", table.name, table.needs_decoder, table.prop_count));
    for prop in table.prop_list {
        // this is fun
        let mut base_str = "                                                                                                         ".to_string();
        
        let type_len = match prop.send_prop_type {
            SendPropType::Int => 3,
            SendPropType::Float | SendPropType::Array => 5,
            SendPropType::String => 6,
            SendPropType::Vector2 | SendPropType::Vector3 => 7,
            SendPropType::DataTable => 9,
        };
        base_str.replace_range(0..type_len, prop.send_prop_type.clone().into());

        base_str.replace_range(15..15 + prop.name.len(), &prop.name);

        if prop.send_prop_type == SendPropType::DataTable || prop.flags.contains(PropFlag::Exclude) {
            base_str.replace_range(46..46 + prop.exclude_dt_name.clone().unwrap().len(), &prop.exclude_dt_name.unwrap());
        } else {
            match prop.send_prop_type {
                SendPropType::String | SendPropType::Int | SendPropType::Float | SendPropType::Vector3 | SendPropType::Vector2 => {
                    base_str.replace_range(46..51, "Low: ");
                    base_str.replace_range(52..52 + prop.low_value.unwrap().to_string().len(), &prop.low_value.unwrap().to_string());
                    
                    base_str.replace_range(62..68, "High: ");
                    base_str.replace_range(69..69 + prop.high_value.unwrap().to_string().len(), &prop.high_value.unwrap().to_string());
                    
                    base_str.replace_range(88..94, "Bits: ");
                    base_str.replace_range(95..95 + prop.num_bits.unwrap().to_string().len(), &prop.num_bits.unwrap().to_string());
                },
                SendPropType::Array => {
                    base_str.replace_range(46..56, "Elements: ");
                    base_str.replace_range(57..57 + prop.num_elements.unwrap().to_string().len(), &prop.num_elements.unwrap().to_string());
                },
                SendPropType::DataTable => {}
            }
        }

        let mut flag_str = "Flags: ".to_string();
        flag_str.push_str(&bitflags_to_string(prop.flags.iter_names()));
        
        base_str.push_str(&flag_str);

        file.write_fmt(format_args!("\n\t\t\t{}", base_str));
    }
}