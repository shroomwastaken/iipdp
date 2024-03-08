use std::fs;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::process::exit;
use std::time::Instant;
use std::ffi::OsStr;

use args::Args;
use structs::demo::Demo;
use structs::demo_header::DemoHeader;
use bitreader::BitReader;

#[macro_use] extern crate enum_primitive;
use bitflags;

// declaring modules
mod structs;
mod bitreader;
mod info_processor;
mod parser;
mod adjust_time;
mod args;

fn main() {
    let args: Args = Args::parse(env::args().collect());

    let path: &Path = Path::new(&args.demo_name);

    if path.is_file() {
        if path.extension().unwrap_or_else(|| {OsStr::new("nope")}) == "dem" {
            let mut main_reader: BitReader = BitReader::new(fs::read(args.demo_name.clone()).unwrap_or_else(|err| {
                println!(r#"Demo file reading failed because of: {} ¯\_(ツ)_/¯"#, err);
                io::stdin().read_line(&mut String::new()).unwrap();
                exit(1);
            }));
            println!("Parsing...\n");
            let start_time = Instant::now();
        
            let mut demo = Demo::new();
        
            demo.header = DemoHeader::parse(&mut main_reader);

            if demo.header.demo_file_stamp != "HL2DEMO" {
                println!("Invalid demo file");
                io::stdin().read_line(&mut String::new()).unwrap();
                return;
            }

            demo.data_manager.get_info_from_header(&demo.header);
            demo.data_manager.dumping = args.dump;
            demo.packets = parser::get_packets(&mut main_reader, &mut demo);
        
            if !args.dump {
                info_processor::print_header_info(demo);
                println!("\nParsed in {:?}", Instant::now().duration_since(start_time));
            } else {
                if args.fc {
                    info_processor::dump_flattened_classes(&args.demo_name, demo.data_manager.dt_mgr.prop_lookup);
                    println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
                } else if args.v {
                    info_processor::verifier_dump(&args.demo_name, demo, &None);
                    println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
                } else {
                    info_processor::dump_file(&args.demo_name, demo);
                    println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
                }
            }
        } else {
            println!("Invalid file!");
            io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
    } else if path.is_dir() {
        let mut vdumpfile: Option<&File> = None;
        let creator: File;
        if args.v && args.dump {
            let new_path = path.to_str().unwrap().to_string().trim_end_matches(".dem").to_owned() + "-vdump.txt";
            creator = File::create(new_path.clone()).unwrap_or_else(|_| {
                println!("Error when creating vdump file :(");
                exit(1);
            });
            println!("File created at: {}\n", new_path);
            vdumpfile = Some(&creator);
        }
        let start_time = Instant::now();
        let mut files: Vec<_> = fs::read_dir(path).unwrap()
                                              .map(|r| r.unwrap())
                                              .collect();
        files.sort_by_key(|dir| dir.path());


        let mut total_measured_ticks: i32 = 0;
        let mut total_measured_time: f32 = 0.0;
        
        let mut total_adjusted_ticks: i32 = 0;
        let mut total_adjusted_time: f32 = 0.0;

        for file in files {
            if file.path().extension().unwrap_or_else(|| {OsStr::new("nope")}) == "dem" {
                println!("\n\nFile Name: {:?}", file.file_name());
                
                let mut main_reader: BitReader = BitReader::new(fs::read(file.path()).unwrap_or_else(|err| {
                    println!(r#"Demo file reading failed because of: {} ¯\_(ツ)_/¯"#, err);
                    io::stdin().read_line(&mut String::new()).unwrap();
                    exit(1);
                }));
            
                println!("Parsing...\n");
                let start_time = Instant::now();
            
                let mut demo = Demo::new();

                demo.header = DemoHeader::parse(&mut main_reader);

                if demo.header.demo_file_stamp != "HL2DEMO" {
                    println!("Invalid demo file");
                    io::stdin().read_line(&mut String::new()).unwrap();
                    return;
                }

                demo.data_manager.get_info_from_header(&demo.header);
                demo.data_manager.dumping = args.dump;
                demo.packets = parser::get_packets(&mut main_reader, &mut demo);
            
                if demo.header.demo_file_stamp != "HL2DEMO" {
                    println!("Invalid demo file");
                    io::stdin().read_line(&mut String::new()).unwrap();
                    return;
                }
            
                total_measured_ticks += demo.data_manager.get_measured_ticks_and_time().0;
                total_measured_time += demo.data_manager.get_measured_ticks_and_time().1;

                total_adjusted_ticks += demo.data_manager.get_adjusted_ticks_and_time().0;
                total_adjusted_time += demo.data_manager.get_adjusted_ticks_and_time().1;

                if args.dump {
                    if args.fc {
                        info_processor::dump_flattened_classes(&args.demo_name, demo.data_manager.dt_mgr.prop_lookup);
                        println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
                    } else if args.v {
                        info_processor::verifier_dump(&file.file_name().to_string_lossy().into_owned(), demo, &vdumpfile);
                        println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
                    } else {
                        info_processor::dump_file(&args.demo_name, demo);
                        println!("\nDumped in {:?}", Instant::now().duration_since(start_time));
                    }
                } else {
                    info_processor::print_header_info(demo);
                    println!("\nParsed in {:?}", Instant::now().duration_since(start_time));
                }
            }
        }

        println!("\n\nTotal Measured Ticks: {}", total_measured_ticks);

        let mut minutes = (total_measured_time / 60f32).floor();
        let mut seconds = (total_measured_time - (60f32 * minutes)).floor();
        let mut millis = (total_measured_time - (60f32 * minutes)).fract();

        println!("Total Measured Time: {}:{:02}.{:.0}", minutes, seconds, millis * 1000.0);

        if total_adjusted_ticks != total_measured_ticks {
            println!("\nTotal Adjusted Ticks: {}", total_adjusted_ticks);

            minutes = (total_adjusted_time / 60f32).floor();
            seconds = (total_adjusted_time - (60f32 * minutes)).floor();
            millis = (total_adjusted_time - (60f32 * minutes)).fract();

            println!("Total Adjusted Time: {}:{:02}.{:.0}", minutes, seconds, millis * 1000.0);
        }
            

        println!("\nParsed all files in: {:?}", Instant::now().duration_since(start_time));
    } else {
        println!("Something went wrong with your arguments.");
    }
    
    io::stdin().read_line(&mut String::new()).unwrap();
}
