use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    if let Some(path) = env::args().nth(1) {
        if let Some(f) = env::args().nth(2) {
            let mut count = 0;
            let mut file = File::open(path)?;
            
            let reader = BufReader::new(file);
            for line in reader.lines() {
                
                if let Ok(l) = line {
                
                    let words = l.split_whitespace();
                    for w in words {
                        if f == w {
                            count += 1;
                        }
                    }
                    
                }
            }
            println!("Found: {}, {} times", f, count);
        }
    }
    Ok(())
}
