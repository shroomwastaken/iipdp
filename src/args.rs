const USAGE_TEXT: &str = "
iipdp v0.2.7 made by shroom
Usage:
\tiipdp.exe <DEMO_NAME> [OPTIONS]

Options:
\t-dump: Dump data from demo into a .txt file in the demo's directory
\t-help: Print this message";


pub struct Args {
    pub demo_name: String,
    pub dump: bool,
}

impl Args {
    pub fn parse(args: Vec<String>) -> Self {
        let options: Vec<String> = vec!["-help".to_string(), "-dump".to_string()];
        if args.len() == 1 || args.contains(&"-help".to_string()) || options.contains(&args[1]){
            println!("{}", USAGE_TEXT);
            std::io::stdin().read_line(&mut String::new()).unwrap();
            std::process::exit(0);
        }
        
        Self { demo_name: args[1].clone(), dump: args.contains(&"-dump".to_string()) }
    }
}