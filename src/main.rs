use std::fs;
use std::env;
use std::io;
use std::process::exit;
use iipdp;
mod info_processor;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut dumping: bool = false;

    if args.len() == 1 {
        println!("IIPDP v0.1 made by shroom\nUsage: iipdp <demo file> [-dump]");
        println!("-dump will output all of the available demo file information into a text file in the working directory");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    } else if args.len() == 2  {
        println!("IIPDP v0.1 made by shroom\nFile Name: {}\n", &args[1])
    } else if args.len() == 3 && args[2] == "-dump" {
        dumping = true;
    } else {
        println!("IIPDP v0.1 made by shroom\nInvalid arguments!");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }
    
    let contents = fs::read(&args[1]).unwrap_or_else(|err| {
        println!(r#"Demo file reading failed because of: {} ¯\_(ツ)_/¯"#, err);
        io::stdin().read_line(&mut String::new()).unwrap();
        exit(1);
    });

    let header_info = iipdp::get_header_info(&contents);

    if header_info["DemoFileStamp"] != "HL2DEMO" {
        println!("Invalid demo file");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    let wrapped_messages = iipdp::get_messages(&contents);

    let mut sorted_keys: Vec<i32> = wrapped_messages.keys().cloned().collect();
    sorted_keys.sort();

    if !dumping {
        info_processor::print_header_info(header_info, sorted_keys);
    } else {
        info_processor::dump_file(&args[1], header_info, wrapped_messages, sorted_keys);
    }

    io::stdin().read_line(&mut String::new()).unwrap();
}
