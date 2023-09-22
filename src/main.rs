use std::fs;
use std::env;
use std::io;
use iipdp;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("you didnt provide a demo file");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }
    
    let contents = fs::read(&args[1]).expect(r#"demo file reading failed ¯\_(ツ)_/¯"#);

    let header_info = iipdp::get_header_info(&contents);

    if iipdp::recognize_demo(&contents) {
        println!("demo recognized!");
    } else {
        println!("invalid demo file");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    println!("File Stamp: {:?}", header_info["DemoFileStamp"]);
    println!("Demo Protocol: {:?}", header_info["DemoProtocol"]);
    println!("Network Protocol: {:?}", header_info["NetworkProtocol"]);
    println!("Server Name: {:?}", header_info["ServerName"]);
    println!("Client Name: {:?}", header_info["ClientName"]);
    println!("Map Name: {:?}", header_info["MapName"]);
    println!("Game Directory: {:?}", header_info["GameDirectory"]);
    println!("Playback Time: {:?}", header_info["PlaybackTime"]);
    println!("Playback Ticks: {:?}", header_info["PlaybackTicks"]);
    println!("Playback Frames: {:?}", header_info["PlaybackFrames"]);
    println!("Sign On Length: {:?}", header_info["SignOnLength"]);

    io::stdin().read_line(&mut String::new()).unwrap();
}
