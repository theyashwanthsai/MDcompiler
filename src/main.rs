use::std::io::{BufRead, BufReader};
use::std::fs::File;
use::std::path::Path;
use::std::io::Write;

fn parse_md(_filename: &str){
    print_short_banner();
    println!("[ INFO ] Trying to parse {}....", _filename);
    
    let input_filename = Path::new(_filename);

    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file");

    let mut h1flag: bool = false;
    let mut pflag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);


    for line in reader.lines(){
        let line_content = line.unwrap();
        // println!("{}", line_content);
        let mut first_char: Vec<char> = line_content.chars().take(1).collect();
        let mut output_line = String::new();


        match first_char.pop() {
            Some('#') => {
                // First order heading (h1)
                if pflag{
                    pflag = false;
                    output_line.push_str("</p>\n");
                }
                if h1flag{
                    h1flag = false;
                    output_line.push_str("</h1>\n");
                }

                h1flag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_content[2..]);
            }
            _ => {
                // not #, could be anything
                if !pflag{
                    pflag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_content);
            }
        }

        if pflag{
            pflag = false;
            output_line.push_str("</p>\n");
        }

        if h1flag{
            h1flag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n"{
            tokens.push(output_line);
        }
        
    }

    // Debug for testing
    // for t in tokens{
    //     println!("{}", t);
    // }

    let mut output_filename  = String::from(&_filename[.._filename.len()-3]);
    output_filename.push_str(".html");

    let mut output_file = File::create(output_filename).expect("[ ERROR ] Could not create html!");

    for line in tokens{
        output_file.write_all(line.as_bytes()).expect("[ ERROR ] Could not write output file!");
    }

    println!("[ INFO ] Parsing complete!");


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