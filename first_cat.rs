///# Cat implementation with Rust.
//
//Modules loaded with brief description:
use std::env;                   // To read command line arguments.
use std::io::BufReader;         // To create a read buffer.
use std::fs::File;              // To open the file.
use std::io::prelude::*;        // To add some I/O traits.


fn main() {
// Getting the list of command line arguments.
    let args: Vec<String> = env::args().

    if args.len() == 2 {
        // Opening the file.
        let f = File::open(&args[1]).unwrap();
        let reader = BufReader::new(f);
        // Printing each line.
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}
