use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // --snip--
    let files = fs::read_dir("assets/sample_txts").unwrap(); //Get the sample texts directory
    for file in files {
        println!("Name: {}", path.unwrap().path().display()) //Print out each .txt file in the sample txts directory
    }

    println!("Enter the name of any of the above text files to use: (To use another file,\
    place it in the \"assets/sample_txts\" folder and re-launch the application."); //Prompt the user to enter the name of a file to count words in

    let filename = "assets/sample_txts/shrek.txt";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
    println!("Pls drink bleach and kill yourself.");
}