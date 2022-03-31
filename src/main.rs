// vim:nowrap:
// https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
// https://forge.rust-lang.org/release/platform-support.html
// source ~/.cargo/env
// cargo build --release
// ~/.cargo/bin/cargo build --release --target x86_64-unknown-linux-musl

/*
NOTE
[source](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)
    The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
    The iterator returned by iter will yield &T, by convention.
    The iterator returned by iter_mut will yield &mut T, by convention.
*/

// [](https://users.rust-lang.org/t/need-help-to-convert-example-to-rust/2658/2)


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use clap::{
    Parser,
    Subcommand,
};
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs;
use std::io::{
    BufRead,
    BufReader,
    self,
};
use std::path::Path;
use std::str::FromStr;
 
extern crate regex;
use regex::Regex;

//extern crate Coutner;
use counter::Counter;


//type Counts = BTreeMap<String, u32>;
type Counts = HashMap<String, u32>;
//let mut delme = Counts::new();



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



/// Return an open file or stdin if no filename.
fn get_reader(filename: &Option<String>) -> Box<dyn BufRead>
{
    // https://stackoverflow.com/a/49964042
    // https://www.reddit.com/r/rust/comments/jv3q3e/comment/gci1mww/?utm_source=share&utm_medium=web2x&context=3
    match filename {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) if filename == "-"  => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    }
}



fn word_count1(filename: &Option<String>) -> HashMap<String, u32> {
    // Default implementation with for loops.
    let mut counts : HashMap<String, u32> = HashMap::new();
    //let mut counts : BTreeMap<String, u32> = BTreeMap::new();

    // Consumes the iterator, returns an (Optional) String
    for line_ in get_reader(filename).lines() {
        if let Ok(line) = line_ {
            //let words: Vec<&str> = line.split(char::is_whitespace).collect();
            for word in line.split(char::is_whitespace) {
                // Filter out multiple spaces delimiting to empty strings.
                if word.len() > 0 {
                    *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
                }
            }
        }
    }

    counts
}



/// Good old double for-loop.
fn word_count1_bt(filename: &Option<String>) -> BTreeMap<String, u32> {
    // Default implementation with for loops.
    //let mut counts : HashMap<String, u32> = HashMap::new();
    let mut counts : BTreeMap<String, u32> = BTreeMap::new();

    // Consumes the iterator, returns an (Optional) String
    for line_ in get_reader(filename).lines() {
        if let Ok(line) = line_ {
            //let words: Vec<&str> = line.split(char::is_whitespace).collect();
            for word in line.split(char::is_whitespace) {
                // Filter out multiple spaces delimiting to empty strings.
                if word.len() > 0 {
                    *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
                }
            }
        }
    }

    counts
}



/// While - for-loop.
// TODO someting is wrong with this version as it takes forever.
fn word_count1a(filename: &Option<String>) -> HashMap<String, u32> {
    // Default implementation with for loops.
    let mut counts : HashMap<String, u32> = HashMap::new();
    //let mut counts : BTreeMap<String, u32> = BTreeMap::new();

    // Consumes the iterator, returns an (Optional) String
    while let Some(std::result::Result::Ok(line)) = get_reader(filename).lines().next() {
        //let words: Vec<&str> = line.split(char::is_whitespace).collect();
        for word in line.split(char::is_whitespace) {
            // Filter out multiple spaces delimiting to empty strings.
            if word.len() > 0 {
                *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
            }
        }
    }

    counts
}



/// for-loop match.
fn word_count2(filename: &Option<String>) -> HashMap<String, u32> {
    let mut counts : HashMap<String, u32> = HashMap::new();
    //let mut counts : BTreeMap<String, u32> = BTreeMap::new();

    for line in get_reader(filename).lines() {
        match line {
            Ok(line_) => {
                // The following line makes the code as slow as word_count3.
                //let tokens = line_.split(char::is_whitespace).map(String::from).collect::<Vec<String>>();
                // The following line is slower than word_count1 but much faster than word_count3.
                //let tokens: Vec<&str> = line_.split(char::is_whitespace).collect();
                for token in line_.split(char::is_whitespace) {
                    // Filter out multiple spaces delimiting to empty strings.
                    if token.len() > 0 {
                        *counts.entry(token.to_owned()).or_insert(0u32) += 1u32;
                    }
                }
            }
            Err(e) => { 
                println!("Error reading file: {}", e);
                panic!("Error!");
            }
        }
    }

    counts
}



/// Fluent notation.
fn word_count3(filename: &Option<String>) -> HashMap<String, u32> {
    let counts :HashMap<String, u32> = get_reader(filename)
        // BufRead.lines().  The iterator returned from this function will yield instances of io::Result<String>.
        .lines()

        // Result.unwrap().  Returns the contained Ok value, consuming the self value.
        //.map(|r: io::Result<String>| r.unwrap())
        .map(Result::unwrap)

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        .flat_map(|l: String| {
            // pub fn String::split<'a, P>(&'a self, pat: P) -> Split<'a, P>
            /*
            l.split(char::is_whitespace)
                .map(String::from)
                .collect::<Vec<_>>()
            */
            l.split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
        })

        .fold(HashMap::new(), |mut counts: HashMap<String, u32>, word: String| {
            *counts.entry(word.to_owned()).or_insert(0) += 1;
            counts
        })
        ;

    //println!("{:?}", counts);

    counts
}



/// Fluent notation and folding.
fn word_count4(filename: &Option<String>) -> HashMap<String, u32> {
    //let mut counts : HashMap<String, u32> = HashMap::new();

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
    let counts :HashMap<String, u32> = get_reader(filename)
        // BufRead.lines().  The iterator returned from this function will yield instances of io::Result<String>.
        .lines()

        // Result.unwrap().  Returns the contained Ok value, consuming the self value.
        //.map(|r: io::Result<String>| r.unwrap())
        .map(Result::unwrap)

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        // pub fn String::split<'a, P>(&'a self, pat: P) -> Split<'a, P>
        .flat_map(|l: String| {
            l.split(char::is_whitespace)
                .map(String::from)
                .collect::<Vec<_>>()
        })

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
/// Using a regular expression.
fn word_count5(filename: &Option<String>) -> HashMap<String, u32> {
    // Example from rosetta code.
    let word_regex = Regex::new("(?i)[^ ]+").unwrap();

    let mut words : HashMap<String, u32> = HashMap::new();
    for line in get_reader(filename).lines() {
        word_regex
            /*
             * https://docs.rs/regex/1.3.9/regex/struct.Regex.html#method.find_iter
             * pub fn Regex::find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't>
             * Returns an iterator for each successive non-overlapping match in text, returning the
             * start and end byte indices with respect to text.
             */
            .find_iter(&line.expect("Read error"))
            /*
             * https://docs.rs/regex/1.3.9/regex/struct.Match.html#method.as_str
             * pub fn Match<'t>::as_str(&self) -> &'t str
             * Returns the matched text.
             */
            .map(|m| m.as_str())
            .for_each(|word: &str| {
                /*
                 * https://doc.rust-lang.org/std/primitive.str.html#method.to_owned
                 * pub fn str::to_owned(&self) -> String
                 * Creates owned data from borrowed data, usually by cloning.
                 */
                *words.entry(word.to_owned()).or_insert(0u32) += 1u32;
            });
    }

    words
}



/// Using a fluent notation of iterators and a global counts.
fn word_count6(filename: &Option<String>) -> HashMap<String, u32> {
    let mut counts :HashMap<String, u32> = HashMap::new();
    get_reader(filename)
        // BufRead.lines().  The iterator returned from this function will yield instances of io::Result<String>.
        .lines()

        // Result.unwrap().  Returns the contained Ok value, consuming the self value.
        .map(Result::unwrap)

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        .flat_map(|l: String| {
            // pub fn String::split<'a, P>(&'a self, pat: P) -> Split<'a, P>
            l.split(char::is_whitespace)
                .map(String::from)
                .collect::<Vec<_>>()
        })

        .for_each(|word: String| {
            *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
        })
        ;

    //println!("{:?}", counts);

    counts
}



/// Fluent notation reducing counters.
fn word_count7(filename: &Option<String>) -> HashMap<String, u32> {
    let counts = get_reader(filename)
        .lines()
        .map(Result::unwrap)
        .map(|l: String| l.split_whitespace().map(String::from).collect::<Counter<_>>())
        .reduce(|a, b| a + b);

    HashMap::<String, u32>::new()
}



// [Creating word iterator from line iterator](https://stackoverflow.com/a/53606081)
/// Using a fluent notation of iterators and a global counts and for_each.
fn word_count8(filename: &Option<String>) -> HashMap<String, u32> {
    let mut counts :HashMap<String, u32> = HashMap::new();
    get_reader(filename)
        // BufRead.lines().  The iterator returned from this function will yield instances of io::Result<String>.
        .lines()

        // Result.unwrap().  Returns the contained Ok value, consuming the self value.
        .map(Result::unwrap)

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        .flat_map(|l: String| {
            l.split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .into_iter()
        })

        .for_each(|word: String| {
            *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
        })
        ;

    //println!("{:?}", counts);

    counts
}






/// Helper function to display the counts.
fn print_counts<I, K, V>(counts : I)
    where
        I: Iterator<Item=(K, V)>,
        K: ::std::fmt::Display,
        V: ::std::fmt::Display,
{
    for (key, value) in counts {
        println!("{}\t{}", key, value);
    }
}





#[derive(Parser)]
#[clap(name = "Get Vocabulary")]
#[clap(author, version, about="Get vocabulary", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}



#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Good old double for-loop.
    wc1 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    /*
    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Word count1 BTree
    wc1_bt {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },
    */

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// While - for-loop.
    wc1a {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// for-loop match.
    wc2 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Word count3
    wc3 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Fluent notation and folding.
    wc4 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Word count using a regular expression.
    wc5 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Using a fluent notation of iterators and a global counts.
    wc6 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Fluent notation reducing counters.
    wc7 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Using a fluent notation of iterators and a global counts and for_each.
    wc8 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },
}





fn main() {
    let args = Cli::parse();
    let counts = match &args.command {
        Commands::wc1 {filename} => word_count1(filename),
        //Commands::wc1_bt {filename} => word_count1_bt(filename),
        Commands::wc1a {filename} => word_count1a(filename),
        Commands::wc2 {filename} => word_count2(filename),
        Commands::wc3 {filename} => word_count3(filename),
        Commands::wc4 {filename} => word_count4(filename),
        Commands::wc5 {filename} => word_count5(filename),
        Commands::wc6 {filename} => word_count6(filename),
        Commands::wc7 {filename} => word_count7(filename),
        Commands::wc8 {filename} => word_count8(filename),
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
