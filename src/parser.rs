use crate::structs::cmd_info::CmdInfo;
use crate::structs::data_manager::DataManager;
use crate::structs::demo::Demo;
use crate::structs::net_svc_message::parse;
use crate::structs::packet::Packet;
use crate::structs::packet::PacketDataType;
use crate::structs::packet::PacketType;
use crate::bitreader::BitReader;
use crate::structs::packet_data_types::{PP, ConsoleCmd, UserCmd, SyncTick, StringTables, DataTables, Stop};
use crate::structs::user_cmd_info::UserCmdInfo;

// all information about the .dem file structure was taken from https://nekz.me/dem/demo.html and UntitledParser

/*
    WARNING: BEAUTIFUL CODE AHEAD
*/

// takes file bytes (contents);
// returns a vector of Message;
pub fn get_packets(reader: &mut BitReader, demo: &mut Demo) -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();

    loop {
        let mut cur_packet: Packet = Packet::new();
        let packet_type = reader.read_int(8);
        cur_packet.packet_type = PacketType::from_int(packet_type);
        if cur_packet.packet_type != PacketType::Stop {
            cur_packet.tick = reader.read_int(32);

            cur_packet.data = read_packet_data(reader, cur_packet.packet_type, &mut demo.data_manager);
        } else {
            cur_packet.tick = reader.read_int(24); // last int is 3 bytes for whatever reason

            cur_packet.data = read_packet_data(reader, cur_packet.packet_type, &mut demo.data_manager);

            packets.push(cur_packet);

            return packets;
        }
        
        packets.push(cur_packet);
    }
}

// takes file bytes (contents), start index (start), message type (msg_type);
// returns HashMap<field name: String, value: Box<dyn Any>>;
fn read_packet_data(reader: &mut BitReader, packet_type: PacketType, demo_data_mgr: &mut DataManager) -> PacketDataType {
    let packet_data: PacketDataType;

    match packet_type {
        PacketType::Packet | PacketType::SignOn => {
            let mut data = PP::new();

            data.cmd_info = CmdInfo::parse(reader);
                        
            data.in_sequence = reader.read_int(32);
            data.out_sequence = reader.read_int(32);
            data.size = reader.read_int(32); // in bytes!!!

            data.messages = parse(&mut reader.split_and_skip(data.size as i32 * 8), demo_data_mgr, data.size);

            packet_data = PacketDataType::Packet(data);
        },
        PacketType::ConsoleCmd => {
            let mut data = ConsoleCmd::new();

            data.size = reader.read_int(32);
            data.data = reader.read_ascii_string((data.size * 8) as i32);
            
            packet_data = PacketDataType::ConsoleCmd(data);
        },
        PacketType::DataTables => {
            let mut data = DataTables::new();
            data.size = reader.read_int(32);
            // no parsing yet so just skip
            reader.skip((data.size * 8) as i32);

            packet_data = PacketDataType::DataTables(data);
        },
        PacketType::Stop => {
            packet_data = PacketDataType::Stop(Stop); // stop contains no data
        },
        PacketType::StringTables => {
            let mut data = StringTables::new();
            data.size = reader.read_int(32);
            data.table_count = reader.read_int(8);

            // no parsing yet so just skip
            reader.skip((data.size * 8) as i32);

            packet_data = PacketDataType::StringTables(data);
        },
        PacketType::SyncTick => {
            packet_data = PacketDataType::SyncTick(SyncTick); // synctick also contains no data
        },
        PacketType::Unknown => {
            packet_data = PacketDataType::Unknown;
        },
        PacketType::UserCmd => {
            let mut data = UserCmd::new();

            data.cmd = reader.read_int(32);
            data.size = reader.read_int(32);
            data.data = UserCmdInfo::parse(&mut reader.split_and_skip((data.size * 8) as i32));

            packet_data = PacketDataType::UserCmd(data);
        }
    }

    return packet_data;
}
