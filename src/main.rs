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

    for (name, value) in header_info {
        println!("{:?}: {:?}", name, value)
    }

    io::stdin().read_line(&mut String::new()).unwrap();
}
