
fn usage(){
    println!("mdcompiler, A tiny markdown compiler written in rust by Sai Yashwanth");
    println!("Version {}", get_version());
}

fn get_version() -> u32{
    1000
}

fn main() {
    // println!("Hello, world!");
    usage();
    
}
