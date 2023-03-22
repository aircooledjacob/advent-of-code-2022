use regex::Regex;
use std::ops::RangeInclusive;

use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the fourth day of advent of code
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

    let mut fully_contained_count = 0;

    let re: Regex = Regex::new(r",|-").unwrap();
    for line in raw_lines {
        let split_string: Vec<usize> = re
            .split(&line)
            .map(|string_value| string_value.parse::<usize>().unwrap())
            .collect();

        let range1 = split_string[0]..=split_string[1];
        let range2 = split_string[2]..=split_string[3];

        if ranges_can_fit_within_another(range1, range2) {
            fully_contained_count += 1;
        }
    }

    println!("number of section assignments that are fully overlapped: {fully_contained_count}");
}

fn ranges_can_fit_within_another(
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
) -> bool {
    if range1.contains(&range2.start()) && range1.contains(&range2.end()) {
        return true;
    }
    if range2.contains(&range1.start()) && range2.contains(&range1.end()) {
        return true;
    }

    false
}
