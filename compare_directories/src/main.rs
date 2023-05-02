// So basically, I have inferior memory and I often end up duplicating files on my filesystem
// (mainly .flac and .ogg files between my iPod and my NAS server. As a result of my mediocrity, I
// need an automation that accepts two directories and produces an output of the dupes. MD5
// comparisons FTW )

use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::io::Read;
use std::path::Path;

use colored::*; //Colorized output will help with readability.
use sha2::{Digest, Sha256};
use walkdir::{DirEntry, WalkDir};

//TODO: Complete the logic below

fn main (){
 
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "{}",
            "Usage: cargo run <path/to/first/dir> <path/to/second/dir>".red()
        );
        std::process::exit(1);
    }


    //The script should accept two directories as input "Usage: cargo run <path/to/first/dir <path/to/second/dir>". We can collect the args with env::args().collect().
    let dir1 = &args[1];
    let dir2 = &args[2];

    //The compare_directories function is called with the two directory paths, and the duplicate files are printed with their paths in a colorized format.
    let mut duplicates = compare_directories(dir1, dir2);

    
}

//Rust's HashMap should do the trick. I'll need the file hashes as keys and a vector of file paths as values, then GTFO of Dodge. 
fn compare_directories(dir1: &str, dir2: &str) -> HashMap<String, Vec<String>> {
    let mut file_hashes: HashMap<String, Vec<String>> = HashMap::new();
    //I'll need to iterate through the paths to perform the comparison
    //TODO Study other folks code. 
    file_hashes
}
