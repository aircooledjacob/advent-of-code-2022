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

    let raw_lines: Vec<String> = common::read_file_to_vec_of_strings(&args.filename);

    let tuple_vec = common::d2::parse_lines_to_char_tuples(&raw_lines);

    let mut score: u32 = 0;
    for (opponent_choice, your_choice) in tuple_vec {
        //increase score for outcome
        match (opponent_choice, your_choice) {
            ('A', 'X') => score += 3,
            ('A', 'Y') => score += 6,
            ('A', 'Z') => score += 0,
            ('B', 'X') => score += 0,
            ('B', 'Y') => score += 3,
            ('B', 'Z') => score += 6,
            ('C', 'X') => score += 6,
            ('C', 'Y') => score += 0,
            ('C', 'Z') => score += 3,
            _ => panic!("Unexpected value in first column, acceptable values are: A, B, C"),
        }

        //increase score for choice
        match your_choice {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _ => panic!("unexpected value in second column, acceptable values are: X, Y, Z"),
        }
    }

    println!("Your final score is {score}")
}
