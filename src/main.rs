use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/*
struct PathSizeSummary {
    path: &str;
    size: u64;
}

fn get_path_sizes() -> u64 {
    
}
*/

fn main() {
    println!("dug {}", VERSION);
    
    for argument in env::args().skip(1) {
        println!("{}", argument);
    }
}