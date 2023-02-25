use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the first day of advent of code
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the input file
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let raw_values = common::read_file(&args.filename);

    let largest_calories_sum = find_largest_calories_sum(&raw_values);

    println!("The elf carrying the most calories is carrying {largest_calories_sum} calories.")
}

fn find_largest_calories_sum(input_vec: &Vec<String>) -> u32 {
    let mut current_sum: u32 = 0;
    let mut largest_sum: u32 = 0;

    for str_value in input_vec {
        match str_value.parse::<u32>() {
            Ok(int_value) => {
                current_sum += int_value;
            }
            Err(_) => {
                if current_sum > largest_sum {
                    largest_sum = current_sum;
                }
                current_sum = 0;
            }
        }
    }

    largest_sum
}
