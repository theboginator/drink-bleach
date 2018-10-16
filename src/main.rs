use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // --snip--
    let filename = "assets/sample_txts/shrek.txt";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
    println!("Pls drink bleach and kill yourself.");
}