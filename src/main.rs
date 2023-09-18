use std::fs;
use std::env;
use std::io;
use iipdp;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let contents = fs::read(&args[1]).expect("hi");

    if iipdp::recognize_demo(&contents) {
        println!("demo recognized!");
    } else {
        println!("invalid demo file!")
    }

    println!("{:?}", iipdp::get_client_name(&contents));

    io::stdin().read_line(&mut String::new()).unwrap();
}
