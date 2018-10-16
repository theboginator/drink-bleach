use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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

    let mut wordCount = word_count(&contents);

    for (key, value) in wordCount{
        println!("{} : {}", key, value);
    }

    println!("Pls drink bleach and kill yourself.");
}

fn countWords(mut hashMap: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
    {
        let c = hashMap.entry(word).or_insert(0);
        *c += 1;
    }

    hashMap
}

fn word_count(sentence: &String) -> HashMap<String, u32> {
    let mut hashMap = sentence.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), countWords);
    return hashMap;
}

fn writeResults(mut hashMap: HashMap<String, u32>, filename: &str) {
    let mut outputDir = format!("assets/indexed_text/counted_{}", filename);
    let mut outputFile = File::create(outputDir);

    for (key, value) in hashMap{
        let mut result = format!("{} : {}", key, value);
        write!(outputFile, "{}", result);
    }

}