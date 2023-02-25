use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the second day of advent of code
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the input file
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let raw_lines: Vec<String> = common::read_file(&args.filename);

    let tuple_vec: Vec<(char, char)> = raw_lines
        .iter()
        .map(|line| {
            let split_line = line.split_once(' ').expect("Malformed input");

            (
                split_line.0.chars().collect::<Vec<char>>()[0],
                split_line.1.chars().collect::<Vec<char>>()[0],
            )
        })
        .collect();

    println!("{tuple_vec:?}")
}
