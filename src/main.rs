fn parse_md(filename: &str){
    print_short_banner();
    println!("[ INFO ] Trying to parse {}....", filename);
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