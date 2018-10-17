//This program reads a file name (set by variable "filename") and counts all the words in
//the .txt file. Prints output to the screen. Default text file is shrek.txt (located in assets)
//Jacob Bogner October 2018 v0.89
//Coming soon: implementing printing results to .txt file and prompting user to enter filename via command line.
use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;


fn main() {
    // --snip--
    let files = fs::read_dir("assets/sample_txts").unwrap(); //Get the sample texts directory
    for file in files {
        println!("Name: {}", file.unwrap().path().display()) //Print out each .txt file in the sample txts directory
    }

    println!("Enter the name of any of the above text files to use: (To use another file,\
    place it in the \"assets/sample_txts\" folder and re-launch the application.)"); //Prompt the user to enter the name of a file to count words in

    let filename = "assets/sample_txts/shrek.txt";
    println!("Loading file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut word_count = word_count(&contents);

    for (key, value) in word_count{
        println!("{} : {}", key, value);
    }
}

fn count_words(mut word_map: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
    {
        let c = word_map.entry(word).or_insert(0);
        *c += 1;
    }

    word_map
}

fn word_count(word_string: &String) -> HashMap<String, u32> {
    let mut word_map = word_string.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), count_words);
    return word_map;
}

fn writeResults(mut word_map: HashMap<String, u32>, filename: &str) {
//    let mut output_directory = format!("assets/indexed_text/counted_{}", filename);
//    let mut output_file = File::create(output_directory);

//    for (key, value) in word_map{
//        output_file.write_all("{} : {}", key, value);
//    }
}