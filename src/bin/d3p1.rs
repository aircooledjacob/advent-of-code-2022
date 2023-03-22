use std::collections::HashSet;

use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the third day of advent of code
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

    let mut sum_of_duplicate_priorities: u32 = 0;

    for rucksack in raw_lines {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);

        let duplicates: HashSet<char> = compartment_2
            .chars()
            .filter(|item| compartment_1.contains(*item))
            .collect();

        let duplicate_priorities = common::d3::parse_items_to_priorities(&duplicates);

        sum_of_duplicate_priorities += u32::from(duplicate_priorities.iter().sum::<u8>());
    }

    println!("{sum_of_duplicate_priorities}")
}
