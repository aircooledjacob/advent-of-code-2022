use regex::Regex;
use std::ops::RangeInclusive;

use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the fourth day of advent of code part 2
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

    let re: Regex = Regex::new(r",|-").unwrap();

    let mut overlapping_assignments = 0;

    for line in raw_lines {
        let split_string: Vec<usize> = re
            .split(&line)
            .map(|string_value| string_value.parse::<usize>().unwrap())
            .collect();

        let range1 = split_string[0]..=split_string[1];
        let range2 = split_string[2]..=split_string[3];

        if ranges_have_overlapping_values(range1, range2) {
            overlapping_assignments += 1;
        }
    }

    println!("number of section assignments that have overlapping assignments: {overlapping_assignments}");
}

fn ranges_have_overlapping_values(
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
) -> bool {
    for (e1, e2) in range1.clone().zip(range2.clone()) {
        if range1.contains(&e2) || range2.contains(&e1) {
            return true;
        }
    }

    false
}
