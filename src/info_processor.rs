use crate::structs::demo::Demo;
use crate::structs::packet::PacketType;
use crate::structs::{packet_data_types as pdt, net_svc_message};
use crate::structs::cmd_info::CmdInfo;
use crate::structs::user_cmd_info::UserCmdInfo;
use std::path::Path;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::io;

pub fn print_header_info(demo: Demo) {
    println!("File Stamp: {}", demo.header.demo_file_stamp);
    println!("Demo Protocol: {}", demo.header.demo_protocol);
    println!("Network Protocol: {}", demo.header.network_protocol);
    println!("Server Name: {}", demo.header.server_name);
    println!("Client Name: {}", demo.header.client_name);
    println!("Map Name: {}", demo.header.map_name);
    println!("Game Directory: {}", demo.header.game_directory);
    println!("Playback Time: {:.3}", demo.header.playback_time);
    println!("Playback Ticks: {}", demo.header.playback_ticks);
    println!("Playback Frames: {}", demo.header.playback_frames);
    println!("Sign On Length: {}", demo.header.sign_on_length);

    print!("\n");

    let ticks: f32 = demo.data_manager.last_packet_tick as f32;
    let time: f32 = (&ticks + 1f32) * 0.015;

    println!("Measured Ticks: {}",  (ticks + 1f32) as i32);

    if time < 60f32 {
        println!("Measured Time: {}", format!("{:.3}", time));
    } else {
        let minutes = (time / 60f32).floor();
        let seconds = time - (60f32 * minutes);
        println!("Measured TIme: {}", format!("{}:{:.3}", minutes, seconds));
    }
}

#[allow(unused)]
pub fn dump_file(file_path: &String, demo: Demo) {
    let mut file = fs::File::create(file_path.trim_end_matches(".dem").to_owned() + "-demo_dump.txt").unwrap_or_else( |err| {
        println!("Something went wrong when trying to create the file: {}", err);
        io::stdin().read_line(&mut String::new()).unwrap();
        exit(1);
    });
    
    println!("File created at: {}\n", file_path.trim_end_matches(".dem").to_owned() + "-demo_dump.txt");

    file.write_all("Generated by IIPDP v0.2.2\n".as_bytes());
    file.write_fmt(format_args!("File Name: {}\n", Path::new(&file_path).file_name().unwrap().to_str().unwrap()));
    
    if demo.header.network_protocol < 15 {
        file.write_all("Presumed game: Portal 3420\n\n".as_bytes());
    } else if demo.header.network_protocol == 24 {
        file.write_all("Presumed game: Portal Steampipe\n\n".as_bytes());
    } else if demo.header.network_protocol == 15 {
        file.write_all("Presumed game: Portal 5135 (source unpack)\n\n".as_bytes());
    }
    
    file.write_all(("File Stamp: ".to_owned() + &demo.header.demo_file_stamp + "\n").as_bytes());
    file.write_all(("Demo Protocol: ".to_owned() + &demo.header.demo_protocol.to_string() + "\n").as_bytes());
    file.write_all(("Network Protocol: ".to_owned() + &demo.header.network_protocol.to_string() + "\n").as_bytes());
    file.write_all(("Server Name: ".to_owned() + &demo.header.server_name + "\n").as_bytes());
    file.write_all(("Client Name: ".to_owned() + &demo.header.client_name + "\n").as_bytes());
    file.write_all(("Map Name: ".to_owned() + &demo.header.map_name + "\n").as_bytes());
    file.write_all(("Game Directory: ".to_owned() + &demo.header.map_name + "\n").as_bytes());
    file.write_all(("Playback Time: ".to_owned() + &demo.header.playback_time.to_string() + "\n").as_bytes());
    file.write_all(("Playback Ticks: ".to_owned() + &demo.header.playback_ticks.to_string() + "\n").as_bytes());
    file.write_all(("Playback Frames: ".to_owned() + &demo.header.playback_frames.to_string() + "\n").as_bytes());
    file.write_all(("Sign On Length: ".to_owned() + &demo.header.sign_on_length.to_string() + "\n").as_bytes());

    file.write_all("\n".as_bytes());
    for packet in demo.packets {
        let cur_packet_type = packet.packet_type;
        
        if cur_packet_type == PacketType::SignOn || cur_packet_type == PacketType::Packet {
            let packet_data: pdt::PP = packet.data.into();
            if cur_packet_type == PacketType::SignOn {
                file.write_fmt(format_args!("[{}] SIGNON (1)\n", &packet.tick));
            } else {
                file.write_fmt(format_args!("[{}] PACKET (2)\n", &packet.tick));
            }

            file.write_all("\tCmdInfo:\n".as_bytes());
            let cmd_info: CmdInfo = packet_data.cmd_info;

            file.write_fmt(format_args!("\t\tFlags: {}\n", cmd_info.flags));

            file.write_all("\t\tViewOrigin: ".as_bytes());
            for i in cmd_info.view_origin {
                file.write_fmt(format_args!("{} ", &i));
            }
            file.write_all("\n".as_bytes());

            file.write_all("\t\tViewAngles: ".as_bytes());
            for i in cmd_info.view_angles {
                file.write_fmt(format_args!("{} ", &i));
            }
            file.write_all("\n".as_bytes());

            file.write_all("\t\tLocalViewAngles: ".as_bytes());
            for i in cmd_info.local_view_angles {
                file.write_fmt(format_args!("{} ", &i));
            }
            file.write_all("\n".as_bytes());

            file.write_all("\t\tViewOrigin2: ".as_bytes());
            for i in cmd_info.view_origin2 {
                file.write_fmt(format_args!("{} ", &i));
            }
            file.write_all("\n".as_bytes());

            file.write_all("\t\tViewAngles2: ".as_bytes());
            for i in cmd_info.view_angles2 {
                file.write_fmt(format_args!("{} ", &i));
            }
            file.write_all("\n".as_bytes());

            file.write_all("\t\tLocalViewAngles2: ".as_bytes());
            for i in cmd_info.local_view_angles2 {
                file.write_fmt(format_args!("{} ", &i));
            }
            file.write_all("\n".as_bytes());

            file.write_fmt(format_args!("\tInSequence: {}\n", packet_data.in_sequence));
            file.write_fmt(format_args!("\tOutSequence: {}\n", packet_data.out_sequence));
            file.write_fmt(format_args!("\tData Size (bytes): {}", packet_data.size));
            net_svc_message::write_msg_data_to_file(&mut file, packet_data.messages, &demo.data_manager);
        } else if cur_packet_type == PacketType::SyncTick {
            let _ = file.write_fmt(format_args!("[{}] SYNCTICK (3)\n", packet.tick));
        } else if cur_packet_type == PacketType::ConsoleCmd {
            let packet_data: pdt::ConsoleCmd = packet.data.into();
            file.write_fmt(format_args!("[{}] CONSOLECMD (4)\n", packet.tick));
            file.write_fmt(format_args!("\tData Size (bytes): {}\n", packet_data.size));
            file.write_fmt(format_args!("\tData:\n\t\t{}\n", packet_data.data));
        } else if cur_packet_type == PacketType::UserCmd {
            let packet_data: pdt::UserCmd = packet.data.into();
            file.write_fmt(format_args!("[{}] USERCMD (5)\n", packet.tick));
            file.write_fmt(format_args!("\tCmd: {}\n", packet_data.cmd));
            file.write_fmt(format_args!("\tData Size (bytes): {}\n", packet_data.size));
            
            let user_cmd_info: UserCmdInfo = packet_data.data;

            file.write_all("\tUserCmdInfo:\n".as_bytes());
            // theres still spaghetti but this time i dont *think* theres a better way to do this
            file.write_fmt(format_args!("\t\tCommand Number: {}\n", user_cmd_info.command_number.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
            file.write_fmt(format_args!("\t\tTick Count: {}\n", user_cmd_info.tick_count.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));

            file.write_all("\t\tView Angles: ".as_bytes());
            file.write_fmt(format_args!("{} ", user_cmd_info.view_angles_x.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
            file.write_fmt(format_args!("{} ", user_cmd_info.view_angles_y.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
            file.write_fmt(format_args!("{}\n", user_cmd_info.view_angles_z.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));

            file.write_all("\t\tMovement: ".as_bytes());
            file.write_fmt(format_args!("{} ", user_cmd_info.forward_move.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
            file.write_fmt(format_args!("{} ", user_cmd_info.side_move.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
            file.write_fmt(format_args!("{}\n", user_cmd_info.up_move.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));

            file.write_fmt(format_args!("\t\tButtons: {}\n", user_cmd_info.buttons.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));
            file.write_fmt(format_args!("\t\tImpulse: {}\n", user_cmd_info.impulse.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}))); 

            file.write_fmt(format_args!("\t\tWeapon Select, Subtype: {}, {}\n", user_cmd_info.weapon_select.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}), 
                                                                                user_cmd_info.weapon_subtype.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));

            file.write_fmt(format_args!("\t\tMouse Dx, Dy: {}, {}\n", user_cmd_info.mouse_dx.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()}),
                                                                    user_cmd_info.mouse_dy.map(|i| {i.to_string()}).unwrap_or_else(|| {"Null".to_string()})));

        } else if cur_packet_type == PacketType::DataTables {
            let packet_data: pdt::DataTables = packet.data.into();
            file.write_fmt(format_args!("[{}] DATATABLES (6)\n", packet.tick));
            file.write_fmt(format_args!("\tData Size (bytes): {}\n", packet_data.size));
            file.write_all("\tNO DATA AVAILABLE YET\n".as_bytes());
            
        } else if cur_packet_type == PacketType::Stop {
            file.write_fmt(format_args!("[{}] STOP (7)\n", packet.tick));
        } else if cur_packet_type == PacketType::StringTables {
            let packet_data: pdt::StringTables = packet.data.into();
            file.write_fmt(format_args!("[{}] STRINGTABLES (8)\n", packet.tick));
            file.write_fmt(format_args!("\tData Size (bytes): {}\n", packet_data.size));
            file.write_all("\tNO DATA AVAILABLE YET\n".as_bytes());
        } else if cur_packet_type == PacketType::Unknown {
            file.write_fmt(format_args!("[{}] Unknown packet type (most likely a bug)\n", packet.tick));
        }
        file.write_all("\n".as_bytes());
    }
    println!("Dumping done!");
}
