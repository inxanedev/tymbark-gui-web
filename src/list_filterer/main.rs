mod lib;

use std::{fs, collections::HashSet};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    filename: String,
    #[clap(short, long)]
    output: String
}

pub fn parse(line: &String, disallowed_words: &[&str], chars_to_remove: &str) -> String {
    let mut current = line.clone();

    // Filter words

    let words = current.split(" ").collect::<Vec<&str>>();
    if words.iter().any(|word| disallowed_words.contains(word)) {
        let found_word = disallowed_words.iter().find(|x| words.contains(x)).unwrap();
        let index = words.iter().position(|x| x == found_word).unwrap();

        let words_up_to_index = &words.as_slice()[0..index]; 

        current = words_up_to_index.join(" ")
    }

    // Remove disallowed characters

    chars_to_remove.chars().for_each(|c| {
        current = current.replace(c, "");
    });

    current
}

fn main() {
    tymbark_gui::logging::setup_logger();
    let args = Args::parse();

    log::info!("Reading file...");
    let lines: Vec<String> = fs::read_to_string(args.filename).unwrap().split('\n').map(|x| x.to_owned()).collect();


    let disallowed_words = vec![
        "lub"
    ];


    let mut output_set: HashSet<String> = HashSet::new();
    log::info!("Filtering and parsing list...");
    for line in lines {
        output_set.insert(parse(&line, &disallowed_words, "?"));
    }

    log::info!("Writing parsed list to output file...");
    fs::write(args.output, output_set.iter().fold(String::default(), |sum, x| {
        sum + x + "\n"
    })).unwrap();
}