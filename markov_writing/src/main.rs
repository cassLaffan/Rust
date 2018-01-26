extern crate rand;

use std::io::prelude::*;
use std::io::Write;
use std::fs::File;
use std::io::BufReader;
//I will use snake case because the compiler keeps yelling at me
//this project will sample .txt files and create a .txt file
//that writes a new text based on the patterns of the old .txt
fn main() {
    println!("Monkies, go!!");
    //open the file that you wanna read
    let mut file = File::open("Shake.txt").expect("Cannot open file!");

    //read the file data in bytes then into a string
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);

    //create a new, blank text file, where our new stuff will be added
    let mut new_work = File::create("NewBook.txt");
    new_work.write_all(buf_reader); //check later? library missing, I think?

}
