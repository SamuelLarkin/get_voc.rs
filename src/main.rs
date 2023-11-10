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


/*
NOTE
[Converting &str: to_string vs to_owned (with two benchmarks)]
(https://medium.com/@ericdreichert/converting-str-to-string-vs-to-owned-with-two-benchmarks-a66fd5a081ce#)
You should always be using to_owned(). to_string() is the generic conversion to a String from any
type implementing the ToString trait. It uses the formatting functions and therefor might end up
doing multiple allocations and running much more code than a simple to_owned() which just allocates
a buffer and copies the literal into the buffer.
*/

/*
NOTE
[What is the idiomatic way to convert &str to String?]
(https://users.rust-lang.org/t/what-is-the-idiomatic-way-to-convert-str-to-string/12160/7)
* to_owned: I have a borrowed object and I want an owned version
* to_string: I want the textual representation of something
* into (or String::from): I want a generic type conversion
* format!: I want a textual representation of something in a particular representation. Basically a
  fancy way of calling to_string (to_string is implemented generically for Display, the canonical
  way of using Display is through format!)
*/


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


type Counts = HashMap<String, u32>;
//type Counts = BTreeMap<String, u32>;   // Almost twice slower than HashMap



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



/// Default implementation with for loops.
fn worc_count_for_for(filename: &Option<String>) -> Counts {
    let mut counts = Counts::new();

    // Consumes the iterator, returns an (Optional) String
    for line_ in get_reader(filename).lines() {
        if let Ok(line) = line_ {
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
fn word_count_while_for(filename: &Option<String>) -> Counts {
    // Default implementation with for loops.
    let mut counts = Counts::new();

    // Consumes the iterator, returns an (Optional) String
    let mut rdr = get_reader(filename).lines();   // This CANNOT be part of the while statement.
    while let Some(std::result::Result::Ok(line)) = rdr.next() {
        for word in line.split(char::is_whitespace) {
            // Filter out multiple spaces delimiting to empty strings.
            if word.len() > 0 {
                *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
            }
        }
    }

    counts
}



/// FASTEST OVERALL
/// Buffer - While - for-loop.
fn word_count_buffer_while_for(filename: &Option<String>) -> Counts {
    // Default implementation with for loops.
    let mut counts = Counts::new();

    // Consumes the iterator, returns an (Optional) String
    let mut rdr = get_reader(filename);   // This CANNOT be part of the while statement.
    let mut line = String::with_capacity(1024);
    while let Ok(read) = rdr.read_line(&mut line) {
        if read == 0 {
            break;
        }
        for word in line.split(char::is_whitespace) {
            // Filter out multiple spaces delimiting to empty strings.
            if word.len() > 0 {
                *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
            }
        }
        line.clear();
    }

    counts
}



/// for-loop match.
fn word_count_for_match_for(filename: &Option<String>) -> Counts {
    let mut counts = Counts::new();

    for line in get_reader(filename).lines() {
        match line {
            Ok(line_) => {
                // The following line makes the code as slow as word_count_fluent_1.
                //let tokens = line_.split(char::is_whitespace).map(String::from).collect::<Vec<String>>();
                // The following line is slower than worc_count_for_for but much faster than
                // word_count_fluent_1.
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



/// SLOWEST NON-FLUENT
//[source](http://rosettacode.org/wiki/Word_frequency#Rust)
/// Using a for-loop and a regular expression.
fn word_count_regex_for(filename: &Option<String>) -> Counts {
    // Example from rosetta code.
    let word_regex = Regex::new("(?i)[^ ]+").unwrap();

    let mut words = Counts::new();
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



/// Fluent notation.
fn word_count_fluent_1(filename: &Option<String>) -> Counts {
    let counts: Counts = get_reader(filename)
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
            //l.split_whitespace().map(String::from)
            l.split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
        })

        .fold(Counts::new(), |mut counts: Counts, word: String| {
            *counts.entry(word.to_owned()).or_insert(0) += 1;
            counts
        })
        ;

    //println!("{:?}", counts);

    counts
}



/// Fluent notation and folding.
fn word_count_fluent_2(filename: &Option<String>) -> Counts {
    //let mut counts = Counts::new();

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
    let counts: Counts = get_reader(filename)
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
        .fold(Counts::new(), |mut counts: Counts, word: String| {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            counts
        });

    //println!("{:?}", counts);

    counts
}



/// FASTEST FLUENT
/// Using a fluent notation of iterators and a global counts.
/// This version is slightly faster than word_count_fluent_5
fn word_count_fluent_3(filename: &Option<String>) -> Counts {
    let mut counts = Counts::new();

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
            //l.split_whitespace()
        })

        .for_each(|word: String| {
            // NOTE doing .to_owned() is not needed and it is expansive.
            //*counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
            *counts.entry(word).or_insert(0u32) += 1u32;
        })
        ;

    //println!("{:?}", counts);

    counts
}



/// SLOWEST OVERALL Even slower than Python.
/// Fluent notation reducing counters.
fn word_count_fluent_4(filename: &Option<String>) -> Counts {
    let counts = get_reader(filename)
        .lines()
        .map(Result::unwrap)
        .map(|l: String| l.split_whitespace().map(String::from).collect::<Counter<_>>())
        .reduce(|a, b| a + b);

    // TODO convert Counter to Counts
    Counts::new()
}



// [Creating word iterator from line iterator](https://stackoverflow.com/a/53606081)
/// Using a fluent notation of iterators and a global counts and for_each.
/// comparable to word_count_fluent_3 with subtle differences.
fn word_count_fluent_5(filename: &Option<String>) -> Counts {
    let mut counts = Counts::new();

    get_reader(filename)
        // BufRead.lines().  The iterator returned from this function will yield instances of io::Result<String>.
        .lines()

        // Result.unwrap().  Returns the contained Ok value, consuming the self value.
        .map(Result::unwrap)

        // Iterator.flat_map().  Creates an iterator that works like map, but flattens nested structure.
        .flat_map(|l: String| {
            l.split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })

        .for_each(|word: String| {
            // NOTE doing .to_owned() is not needed and it is expansive.
            //*counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
            *counts.entry(word).or_insert(0u32) += 1u32;
        })
        ;

    //println!("{:?}", counts);

    counts
}



///
fn word_count_fluent_6(filename: &Option<String>) -> Counts {
    // Not sure the result is correct since we are spliting on space and what about newlines?
    let counts = get_reader(filename)
        .split(b' ')
        .map(Result::unwrap)
        .map(String::from_utf8)
        .map(Result::unwrap)
        .fold(Counts::new(), |mut counts: Counts, word: String| {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            counts
        });

    counts
}






/// Helper function to display the counts.
fn print_counts<I, K, V>(counts: I, show_counts: &bool)
    where
        I: Iterator<Item=(K, V)>,
        K: ::std::fmt::Display,
        V: ::std::fmt::Display,
{
    let print = if *show_counts {
        |key, value| { println!("{}\t{}", key, value) }
    }
    else {
        |key, value| { println!("{}", key) }
    };

    for (key, value) in counts {
        print(key, value);
    }
}





#[derive(Parser)]
#[clap(name = "Get Vocabulary")]
#[clap(author, version, about="Get vocabulary", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Show counts
    #[clap(short, long, name="show_counts")]
    show_counts: bool,

    /// topk
    #[clap(short, long, parse(try_from_str))]
    topk: Option<usize>
}



#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help=false)]
    /// Default implementation with for loops.
    /// Good old double for-loop.
    wcff {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// While - for-loop.
    wcwf {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="fastest")]
    /// Buffer - While - for-loop.
    wcbwf {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// for-loop match for-loop.
    wcfmf {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// Using a for-loop and a regular expression.
    wcrf {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// Word count3
    wc_f1 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// Fluent notation and folding.
    wc_f2 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// Using a fluent notation of iterators and a global counts.
    wc_f3 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// Fluent notation reducing counters.
    wc_f4 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    /// Using a fluent notation of iterators and a global counts and for_each.
    wc_f5 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },

    #[clap(arg_required_else_help=false)]
    wc_f6 {
        /// Input file
        #[clap(name="Input file")]
        filename: Option<String>,
    },
}





fn main() {
    let args = Cli::parse();
    let counts = match &args.command {
        Commands::wcff {filename} => worc_count_for_for(filename),
        Commands::wcwf {filename} => word_count_while_for(filename),
        Commands::wcbwf {filename} => word_count_buffer_while_for(filename),
        Commands::wcfmf {filename} => word_count_for_match_for(filename),
        Commands::wcrf {filename} => word_count_regex_for(filename),
        Commands::wc_f1 {filename} => word_count_fluent_1(filename),
        Commands::wc_f2 {filename} => word_count_fluent_2(filename),
        Commands::wc_f3 {filename} => word_count_fluent_3(filename),
        Commands::wc_f4 {filename} => word_count_fluent_4(filename),
        Commands::wc_f5 {filename} => word_count_fluent_5(filename),
        Commands::wc_f6 {filename} => word_count_fluent_6(filename),
    };

    // [Word Frequency](http://rosettacode.org/wiki/Word_frequency#Rust)
    let mut words: Vec<_> = counts.iter().collect();
    words.sort_unstable_by_key(|&(word, count)| (Reverse(count), word));

    if let Some(topk) = args.topk {
        print_counts(words.into_iter().take(topk), &args.show_counts);
    }
    else {
        print_counts(words.into_iter(), &args.show_counts);
    }
}
