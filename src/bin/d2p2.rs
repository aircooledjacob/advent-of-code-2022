use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the second day of advent of code p2
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the input file
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let raw_lines: Vec<String> = common::read_file_to_vec_of_strings(&args.filename);

    let tuple_vec = common::d2::parse_lines_to_char_tuples(&raw_lines);
}
