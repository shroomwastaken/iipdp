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

// declaring modules
mod structs;
mod bitreader;
mod info_processor;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut dumping: bool = false;

    let mut demo: Demo = Demo::new();

    if args.len() == 1 {
        println!("IIPDP v0.2.2 made by shroom\nUsage: iipdp <demo file> [-dump]");
        println!("-dump will output all of the available demo file information into a text file in the working directory");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    } else if args.len() == 2  {
        println!("IIPDP v0.2.2 made by shroom\nFile Name: {}\n", Path::new(&args[1]).file_name().unwrap().to_str().unwrap())
    } else if args.len() == 3 && args[2] == "-dump" {
        dumping = true;
    } else {
        println!("IIPDP v0.2.2 made by shroom\nInvalid arguments!");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }
    
    let mut main_reader: BitReader = BitReader { contents: fs::read(&args[1]).unwrap_or_else(|err| {
        println!(r#"Demo file reading failed because of: {} ¯\_(ツ)_/¯"#, err);
        io::stdin().read_line(&mut String::new()).unwrap();
        exit(1);
    }), cur_bit_index: 0 };

    println!("Parsing...\n");
    let start_time = Instant::now();

    let mut header: DemoHeader = DemoHeader::new();
    header.parse(&mut main_reader);    

    demo.header = header;

    demo.data_manager.demo_protocol = demo.header.demo_protocol;
    demo.data_manager.network_protocol = demo.header.network_protocol;

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

    io::stdin().read_line(&mut String::new()).unwrap();
}
