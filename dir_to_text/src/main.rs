
use std::fs;

fn main() {
    let mut entries = fs::read_dir("..").unwrap();
    let mut directories: Vec<PathBuf> = Vec::new();


    for entry in entries{
        let entry = entry.unwrap();
        directories.push(entry.path());
        println!("{:?}", entry.path());
    }

}
