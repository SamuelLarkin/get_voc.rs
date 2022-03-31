// vim:nowrap:
// https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
// https://forge.rust-lang.org/release/platform-support.html
// source ~/.cargo/env
// cargo build --release
// ~/.cargo/bin/cargo build --release --target x86_64-unknown-linux-musl

/*
 * [source](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)
 *     The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
 *     The iterator returned by iter will yield &T, by convention.
 *     The iterator returned by iter_mut will yield &mut T, by convention.
 */

// [](https://users.rust-lang.org/t/need-help-to-convert-example-to-rust/2658/2)


use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
 
extern crate regex;
use regex::Regex;


type CountMap = BTreeMap<String, u32>;



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn word_count1(filename: &String) -> HashMap<String, u32> {
    //let filename = "/space/project/portage/models/WMT2020/hsb-de/corpora/preprocessing.multilingual/bpe.02k/teacher.all/train.cs-de.de";
    let mut counts : HashMap<String, u32> = HashMap::new();
    //let mut counts : BTreeMap<String, u32> = BTreeMap::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line_ in lines {
            if let Ok(line) = line_ {
                //println!("{}", line);
                //let words: Vec<&str> = line.split(char::is_whitespace).collect();
                for word in line.split(char::is_whitespace) {
                    // Filter out multiple spaces delimiting to empty strings.
                    if word.len() > 0 {
                        *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
                    }
                }
            }
        }
    }

    counts
}



fn word_count2(filename: &String) -> HashMap<String, u32> {
    let file = File::open(&filename).unwrap();
    let rdr = io::BufReader::new(file);
    //let mut mapped : BTreeMap<String, u32> = BTreeMap::new();
    let mut mapped : HashMap<String, u32> = HashMap::new();
    for line in rdr.lines() {
        match line {
            Ok(line_) => {
                for token in line_.split(char::is_whitespace) {
                    // Filter out multiple spaces delimiting to empty strings.
                    if token.len() > 0 {
                        *mapped.entry(token.to_owned()).or_insert(0) += 1;
                    }
                }
            }
            Err(e) => { 
                println!("Error reading file: {}", e);
                panic!("Error!");
            }
        }
    }
    mapped
}



fn word_count3(filename: &String) -> HashMap<String, u32> {
    let file = File::open(filename);
    let counts :HashMap<String, u32> = io::BufReader::new(file.unwrap())
        // BufRead.lines().  The iterator returned from this function will yield instances of io::Result<String>.
        .lines()

        // Result.unwrap().  Returns the contained Ok value, consuming the self value.
        .map(|r: io::Result<String>| r.unwrap())

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        .flat_map(|l: String| {
            // pub fn String::split<'a, P>(&'a self, pat: P) -> Split<'a, P>
            l.split(char::is_whitespace)
                .map(String::from)
                .collect::<Vec<_>>()
        })

        .fold(HashMap::new(), |mut counts: HashMap<String, u32>, word: String| {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            counts
        });

    //println!("{:?}", counts);

    counts
}



fn word_count4(filename: &String) -> HashMap<String, u32> {
    //let mut counts : HashMap<String, u32> = HashMap::new();

    let file = File::open(filename);
    /*
    let iterator = io::BufReader::new(file.unwrap())
        .lines()
        .map(|r| r.unwrap())
        //.flat_map(|l| l.split(char::is_whitespace))
        .flat_map(|l| l.split_whitespace())
        //.flatten()
        .map(|s| FromStr::from_str(s.as_ref()).ok())
        //.map(|r| r.unwrap())
        //.map(|s| s.clone())
        //.map(|s| s.to_string())
        .collect::<Vec<String>>();
        // value of type `std::vec::Vec<std::string::String>` cannot be built from `std::iter::Iterator<Item=&str>`
    */
    let counts :HashMap<String, u32> = io::BufReader::new(file.unwrap())
        // BufRead.lines().  The iterator returned from this function will yield instances of io::Result<String>.
        .lines()

        // Result.unwrap().  Returns the contained Ok value, consuming the self value.
        .map(|r: io::Result<String>| r.unwrap())

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        // pub fn String::split<'a, P>(&'a self, pat: P) -> Split<'a, P>
        .flat_map(|l: String| l.split(char::is_whitespace).map(String::from).collect::<Vec<_>>())

        //.cloned()

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        // String.split_whitespace().  Splits a string slice by whitespace.  The iterator returned
        // will return string slices that are sub-slices of the original string slice, separated by
        // any amount of whitespace.
        //.flat_map(|l: String| l.split_whitespace())
        //.map(str::to_owned)

        //.map(|s| String::from(s))
        //.map(|s| s.to_string())
        .fold(HashMap::new(), |mut counts: HashMap<String, u32>, word: String| {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            counts
        });

    //println!("{:?}", counts);

    counts
}



//[source](http://rosettacode.org/wiki/Word_frequency#Rust)
fn word_count5(filename: &String) -> HashMap<String, u32> {
    let word_regex = Regex::new("(?i)[^ ]+").unwrap();

    let mut words : HashMap<String, u32> = HashMap::new();
    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines() {
        word_regex
            .find_iter(&line.expect("Read error"))
            .map(|m| m.as_str())
            .for_each(|word| {
                *words.entry(word.to_lowercase()).or_insert(0) += 1;
            });
    }

    words
}



fn print_counts<I, K, V>(counts : I) where
    I: Iterator<Item=(K, V)>,
    K: ::std::fmt::Display,
    V: ::std::fmt::Display,
{
    for (key, value) in counts {
        println!("{}\t{}", key, value);
    }
}





fn main() {
    let args: Vec<String> = env::args().collect();
    let method_id = &args[1].parse::<i32>().unwrap();
    let filename  = &args[2];

    let counts = match method_id {
        1 => word_count1(filename),
        2 => word_count2(filename),
        3 => word_count3(filename),
        4 => word_count4(filename),
        5 => word_count5(filename),
        _ => panic!("Unknown method id: {}", method_id),
    };

    //let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    /*
    for (key, value) in &counts {
        println!("{} {}", key, value);
    }
    */

    // [Word Frequency](http://rosettacode.org/wiki/Word_frequency#Rust)
    let mut words: Vec<_> = counts.iter().collect();
    words.sort_unstable_by_key(|&(word, count)| (Reverse(count), word));
         
    /*
    let n = 10;
    for (word, count) in words.iter().take(n) {
        println!("{:8} {:>8}", word, count);
    }
    */
    /*
    for (word, count) in words.iter() {
        println!("{:8} {:>8}", word, count);
    }
    */
    print_counts(words.into_iter().take(10));
}
