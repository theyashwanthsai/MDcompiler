use::std::io::{BufRead, BufReader};
use::std::fs::File;
use::std::path::Path;

fn parse_md(filename: &str){
    print_short_banner();
    println!("[ INFO ] Trying to parse {}....", filename);
    
    let input_filename = Path::new(filename);

    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file");

    let mut h1flag: bool = false;
    let mut pflag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);


    for line in reader.lines(){
        let line_content = line.unwrap();
        // println!("{}", line_content);
        
        
    }

    
}



fn print_long_banner(){
    print_short_banner();
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Usage: mdcompiler <somefile>.md");
}


fn print_short_banner(){
    println!("{}", get_title());
}


fn get_title() -> String{
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn usage(){
    print_long_banner();
}

fn main(){
    let arg: Vec<String> = std::env::args().collect();
    match arg.len() {
        2 => parse_md(&arg[1]),
        _ => {
            println!("[ ERROR ] No filename specified");
            usage();
            }
    }

}