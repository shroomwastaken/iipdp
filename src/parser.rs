use crate::adjust_time::{try_adjust_for_wakeup, try_adjust_for_glados_death};
use crate::structs::cmd_info::CmdInfo;
use crate::structs::data_manager::DataManager;
use crate::structs::demo::Demo;
use crate::structs::net_svc_message::{parse, NetSvcMessageTypes};
use crate::structs::packet::{Packet, PacketDataType, PacketType};
use crate::bitreader::BitReader;
use crate::structs::packet_data_types::{PP, ConsoleCmd, UserCmd, SyncTick, StringTables, DataTables, Stop};
use crate::structs::stringtable::StringTable;
use crate::structs::user_cmd_info::UserCmdInfo;
use crate::structs::send_table::SendTable;
use crate::structs::utils::{ServerClass, check_for_pause};

// all information about the .dem file structure was taken from https://nekz.me/dem/demo.html and UntitledParser

/*
    WARNING: BEAUTIFUL CODE AHEAD
*/

// takes reader and demo references
// returns a vector of Packet;
pub fn get_packets(reader: &mut BitReader, demo: &mut Demo) -> Vec<Packet> {
    let mut packets: Vec<Packet> = Vec::new();

    loop {
        let mut cur_packet: Packet = Packet::new();
        let packet_type = reader.read_int(8);
        cur_packet.packet_type = PacketType::from_int(packet_type);
        if cur_packet.packet_type != PacketType::Stop {
            cur_packet.tick = reader.read_int(32);
            if cur_packet.tick > 0 {
                demo.data_manager.last_packet_tick = cur_packet.tick;
            }

            cur_packet.data = read_packet_data(reader, cur_packet.packet_type, &mut demo.data_manager, cur_packet.tick);
        } else {
            cur_packet.tick = reader.read_int(24); // last int is 3 bytes for whatever reason

            cur_packet.data = read_packet_data(reader, cur_packet.packet_type, &mut demo.data_manager, cur_packet.tick);

            packets.push(cur_packet);

            return packets;
        }
        
        packets.push(cur_packet);
    }
}

// takes reader reference, the current packet type, a reference to the demos data manager and the current packets tick
// returns packet data wrapped in PacketDataType enum;
fn read_packet_data(reader: &mut BitReader, packet_type: PacketType, demo_data_mgr: &mut DataManager, cur_tick: i32) -> PacketDataType {
    let packet_data: PacketDataType;

    match packet_type {
        PacketType::Packet | PacketType::SignOn => {
            let mut data = PP::new();

            data.cmd_info = CmdInfo::parse(reader);
                        
            data.in_sequence = reader.read_int(32);
            data.out_sequence = reader.read_int(32);
            data.size = reader.read_int(32); // in bytes!!!

            // some optimization
            // if we are paused and past the point of adjustment and not dumping we skip any data after the size variable to go faster
            // same thing for every other packet type
            if !demo_data_mgr.dumping && demo_data_mgr.paused && demo_data_mgr.adj_end_tick != 0 {
                reader.skip(data.size as i32 * 8);
            } else {
                data.messages = parse(&mut reader.split_and_skip(data.size as i32 * 8), demo_data_mgr, data.size);

                if data.messages.iter().find(|m| {m.msg_type == NetSvcMessageTypes::SvcFixAngle}).is_some() && cur_tick != 0 {
                    try_adjust_for_wakeup(&data.messages, cur_tick, demo_data_mgr);
                }
                if data.messages.iter().find(|m| {m.msg_type == NetSvcMessageTypes::SvcSetPause}).is_some() {
                    demo_data_mgr.paused = check_for_pause(&data.messages, demo_data_mgr)
                }    
            }

            packet_data = PacketDataType::Packet(data);
        },
        PacketType::ConsoleCmd => {
            let mut data = ConsoleCmd::new();

            data.size = reader.read_int(32);
            data.data = reader.read_ascii_string((data.size * 8) as i32);

            try_adjust_for_glados_death(&data.data, cur_tick, demo_data_mgr);
            
            packet_data = PacketDataType::ConsoleCmd(data);
        },
        PacketType::DataTables => {
            let mut data = DataTables::new();
            data.size = reader.read_int(32);
            if demo_data_mgr.dumping {
                let index_before_parsing = reader.current;
            
                while reader.read_bool() {
                    let table = SendTable::parse(reader, demo_data_mgr);
                    data.send_tables.push(table);
                }

                data.class_count = reader.read_int(16);
                for _ in 0..data.class_count {
                    let mut server_class = ServerClass::new();
                    // if we didnt get an SvcClassInfo yet (?)
                    if demo_data_mgr.server_class_info == Vec::new() {
                        server_class.datatable_id = reader.read_int(16);
                    } else {
                        server_class.datatable_id = reader.read_int(((demo_data_mgr.server_class_info.len() as f32).log2() + 1f32) as i32);
                    }
                    server_class.class_name = reader.read_ascii_string_nulled();
                    server_class.data_table_name = reader.read_ascii_string_nulled();

                    data.server_classes.push(server_class);
                }
                data.send_table_count = data.send_tables.len() as i32;

                reader.current = index_before_parsing + (data.size * 8) as usize;
                reader.fetch();
            } else {
                reader.skip(data.size * 8);
            }

            packet_data = PacketDataType::DataTables(data);
        },
        PacketType::Stop => {
            packet_data = PacketDataType::Stop(Stop); // stop contains no data
        },
        PacketType::StringTables => {
            let mut data = StringTables::new();
            data.size = reader.read_int(32);
            if demo_data_mgr.dumping {
                let index_before_parsing = reader.current;

                data.table_count = reader.read_int(8);

                for _ in 0..data.table_count {
                    data.tables.push(StringTable::parse(reader));
                }
                
                reader.current = index_before_parsing + (data.size * 8) as usize;
                reader.fetch();
            } else {
                reader.skip(data.size * 8);
            }
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
            if demo_data_mgr.dumping {
                data.data = UserCmdInfo::parse(&mut reader.split_and_skip((data.size * 8) as i32));
            } else {
                reader.skip(data.size * 8);
            }

            packet_data = PacketDataType::UserCmd(data);
        }
    }

    return packet_data;
}
