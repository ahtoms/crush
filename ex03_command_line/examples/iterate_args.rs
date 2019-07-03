use std::env;

fn main() {

    for a in env::args() {
        println!("{}", a);
    }
}
