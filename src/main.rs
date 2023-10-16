use std::fs;
use std::env;
use std::io;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use structs::demo::Demo;
use structs::demo_header::DemoHeader;
use structs::packet::Packet;
use bitreader::BitReader;

#[macro_use] extern crate enum_primitive;
use bitflags;

// declaring modules
mod structs;
mod bitreader;
mod info_processor;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("IIPDP v0.2.4 made by shroom\nUsage: iipdp <demo file> [-dump]");
        println!("-dump will output all of the available demo file information into a text file in the working directory");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    } else if args.len() == 3 && args[2] != "-dump" {
        println!("IIPDP v0.2.4 made by shroom\nInvalid arguments!");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    let path: &Path = Path::new(&args[1]);
    let dumping: bool = args.len() == 3 && args[2] == "-dump";

    if path.is_file() {
        let mut main_reader: BitReader = BitReader { contents: fs::read(&args[1]).unwrap_or_else(|err| {
            println!(r#"Demo file reading failed because of: {} ¯\_(ツ)_/¯"#, err);
            io::stdin().read_line(&mut String::new()).unwrap();
            exit(1);
        }), cur_bit_index: 0 };
    
        println!("Parsing...\n");
        let start_time = Instant::now();
    
        let mut header: DemoHeader = DemoHeader::new();
        header.parse(&mut main_reader);    
    
        let mut demo = Demo::new();
    
        demo.header = header;
    
        demo.data_manager.get_info_from_header(&demo.header);
    
        let packets: Vec<Packet> = parser::get_packets(&mut main_reader, &mut demo);
    
        demo.packets = packets;
    
        if demo.header.demo_file_stamp != "HL2DEMO" {
            println!("Invalid demo file");
            io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
    
        if !dumping {
            info_processor::print_header_info(demo);
            println!("\nParsed in {:?}", Instant::now().duration_since(start_time));
        } else {
            info_processor::dump_file(&args[1], demo);
            println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
        }
    } else if path.is_dir() {
        let start_time = Instant::now();
        let files = fs::read_dir(path).unwrap();
        let mut total_ticks: i32 = 0;
        let mut total_time: f32 = 0.0;
        for f in files {
            let file = f.unwrap();
            if file.path().extension().unwrap() == "dem" {
                println!("\n\nFile Name: {:?}", file.file_name());
                
                let mut main_reader: BitReader = BitReader { contents: fs::read(file.path()).unwrap_or_else(|err| {
                    println!(r#"Demo file reading failed because of: {} ¯\_(ツ)_/¯"#, err);
                    io::stdin().read_line(&mut String::new()).unwrap();
                    exit(1);
                }), cur_bit_index: 0 };
            
                println!("Parsing...\n");
                let start_time = Instant::now();
            
                let mut header: DemoHeader = DemoHeader::new();
                header.parse(&mut main_reader);    
            
                let mut demo = Demo::new();
            
                demo.header = header;
            
                demo.data_manager.get_info_from_header(&demo.header);
            
                let packets: Vec<Packet> = parser::get_packets(&mut main_reader, &mut demo);
            
                demo.packets = packets;
            
                if demo.header.demo_file_stamp != "HL2DEMO" {
                    println!("Invalid demo file");
                    io::stdin().read_line(&mut String::new()).unwrap();
                    return;
                }
            
                total_ticks += demo.data_manager.get_ticks_and_time().0;
                total_time += demo.data_manager.get_ticks_and_time().1;

                if !dumping {
                    info_processor::print_header_info(demo);
                    println!("\nParsed {:?} in {:?}", file.file_name(), Instant::now().duration_since(start_time));
                } else {
                    info_processor::dump_file(&file.path().to_string_lossy().to_string(), demo);
                    println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
                }
            }
        }

        println!("\n\nTotal Measured Ticks: {}", total_ticks);

        let minutes = (total_time / 60f32).floor();
        let seconds = (total_time - (60f32 * minutes)).floor();
        let millis = (total_time - (60f32 * minutes)).fract();

        println!("Total Measured Time: {}:{:02}.{:.0}", minutes, seconds, millis * 1000.0);

        println!("\nParsed all files in: {:?}", Instant::now().duration_since(start_time));
    }
    
    io::stdin().read_line(&mut String::new()).unwrap();
}
