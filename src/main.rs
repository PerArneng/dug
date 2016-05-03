use std::env;
use std::path::Path;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct PathSizeSummary<'a> {
    path: &'a str,
    size: u64
}

fn get_path_sizes<'a, I>(path_strings:I) -> Vec<PathSizeSummary<'a>>
    where I: Iterator<Item=String>  {
        
    for x in path_strings {
        get_path_size(&x);
    }
    return vec![];
}

fn get_path_size(path_string: &str) -> u64 {
    let metadata = fs::metadata(path_string).unwrap();
    println!("{}: {}", path_string, metadata.len());
    
    if metadata.is_file() {
        return metadata.len();
    } else {
        let mut len = 0;
        
        let a = fs::read_dir(path_string).map( |read_dir_iter| 
            read_dir_iter.map( |entry| 
                           entry.map(|e| { e.path().to_str().unwrap() })
            )
        );
        
        /*
        for entry in fs::read_dir(path_string).unwrap() {
            let file_name = entry.unwrap().file_name().to_str();
            println!("{}", file_name.unwrap());
            len += get_path_size(file_name.unwrap());
        }*/
        return len;
    }
}

fn main() {
    println!("dug {}", VERSION);
    
    let size = get_path_sizes(env::args().skip(1));
    
}