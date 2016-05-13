use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
 
    if args.len() == 2 {
        let f = File::open(&args[1]).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}