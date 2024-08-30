// vim:nowrap:
// https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
// https://forge.rust-lang.org/release/platform-support.html
// source ~/.cargo/env
// cargo build --release
// ~/.cargo/bin/cargo build --release --target x86_64-unknown-linux-musl

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use clap::{
    Parser,
    Subcommand,
};
use std::cmp::Reverse;
use get_voc::{
    worc_count_for_for,
    word_count_while_for,
    word_count_buffer_while_for,
    word_count_for_match_for,
    word_count_regex_for,
    word_count_fluent_1,
    word_count_fluent_2,
    word_count_fluent_3_flat_map_for_each,
    word_count_fluent_4_map_reduce,
    word_count_fluent_5,
    word_count_fluent_6,
};



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
        |key, _value| { println!("{}", key) }
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
        Commands::wc_f3 {filename} => word_count_fluent_3_flat_map_for_each(filename),
        Commands::wc_f4 {filename} => word_count_fluent_4_map_reduce(filename),
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
