use std::collections::HashSet;

use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the third day of advent of code part 2
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

    let mut sum_of_badge_priorities: u32 = 0;

    for (i, _) in raw_lines.iter().enumerate().step_by(3) {
        let badge: HashSet<char> = raw_lines[i]
            .chars()
            .filter(|item| raw_lines[i + 1].contains(*item))
            .filter(|item| raw_lines[i + 2].contains(*item))
            .collect();

        println!("{badge:?}");

        let badge_priorities = common::d3::parse_items_to_priorities(&badge);

        sum_of_badge_priorities += u32::from(badge_priorities.iter().sum::<u8>());
    }

    println!("{sum_of_badge_priorities}")
}
