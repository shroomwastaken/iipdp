use crate::bitreader::BitReader;
use crate::enum_primitive::enum_from_primitive;
use crate::structs::data_manager::DataManager;

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

pub struct SendTableProp {
    pub send_prop_type: SendPropType,
    pub name: String,
    pub flags: PropFlag,
    pub exclude_dt_name: Option<String>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub num_bits: Option<i32>,
    pub num_elements: Option<i32>,
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
                SendPropType::String | SendPropType::Int | SendPropType::Float | SendPropType::Vector3 => {
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
        }
    }
}

enum_from_primitive! {
    #[derive(Debug, Clone,PartialEq)]
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

bitflags::bitflags! {
    #[derive(Debug, PartialEq)]
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