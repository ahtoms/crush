use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    if let Some(path) = env::args().nth(1) {
        if let Some(f) = env::args().nth(2) {
            let mut count = 0;
            let mut file = File::open(path)?;
            let mut contents = String::new();
            
            file.read_to_string(&mut contents)?; //Contents in heap now
            
            let words = contents.split_whitespace();
            for w in words {
                if f == w {
                    count += 1;
                }
            }
            println!("Found: {}, {} times", f, count);
        }
    }
    Ok(())
}
