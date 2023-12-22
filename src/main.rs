
fn usage(){

    // let ver: u32;
    let ver = get_version();
    println!("mdcompiler, A tiny markdown compiler written in rust by Sai Yashwanth");
    println!("Version {}", ver);
}

fn get_version() -> u32{
    100
}

fn main() {
    // println!("Hello, world!");
    usage();
    
}
