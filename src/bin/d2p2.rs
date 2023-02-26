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

    let input_tuple_vec = common::d2::parse_lines_to_char_tuples(&raw_lines);

    let converted_tuple_vec: Vec<(char, char)> = input_tuple_vec
        .iter()
        .map(
            |(opponent_choice, outcome)| match (opponent_choice, outcome) {
                ('A', 'X') => ('A', 'Z'),
                ('A', 'Y') => ('A', 'X'),
                ('A', 'Z') => ('A', 'Y'),
                ('B', 'X') => ('B', 'X'),
                ('B', 'Y') => ('B', 'Y'),
                ('B', 'Z') => ('B', 'Z'),
                ('C', 'X') => ('C', 'Y'),
                ('C', 'Y') => ('C', 'Z'),
                ('C', 'Z') => ('C', 'X'),
                _ => panic!("Unexpected char in file."),
            },
        )
        .collect();

    let score = common::d2::calculate_score(&converted_tuple_vec);

    println!("Your final score is {score}")
}
