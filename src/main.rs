use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");


struct PathSizeSummary<'a> {
    path: &'a str,
    size: u64
}

fn get_path_sizes<'a, I>(path_strings:I) -> Vec<PathSizeSummary<'a>>
    where I: Iterator<Item=String>  {
        
    for x in path_strings {
        println!("{}", x)
    }
    return vec![];
}


fn main() {
    println!("dug {}", VERSION);
    
    let size = get_path_sizes(env::args().skip(1));
    
    for argument in env::args().skip(1) {
        println!("{}", argument);
    }
}