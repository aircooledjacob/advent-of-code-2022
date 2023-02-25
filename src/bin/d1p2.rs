use clap::Parser;

use advent_of_code_2022::common;

/// Simple cli tool to solve the first day of advent of code p2
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the input file
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let raw_values = common::read_file_to_vec_of_strings(&args.filename);

    let mut calorie_sums_vec = calculate_calorie_sums(&raw_values);

    calorie_sums_vec.sort();

    let sum_of_top_3_calorie_sums: u32 = calorie_sums_vec.iter().rev().take(3).sum();

    println!("The sum of the top 3 calorie sums is {sum_of_top_3_calorie_sums}")
}

fn calculate_calorie_sums(input_vec: &Vec<String>) -> Vec<u32> {
    let mut current_sum: u32 = 0;
    let mut calorie_sums: Vec<u32> = vec![];

    for str_value in input_vec {
        match str_value.parse::<u32>() {
            Ok(int_value) => {
                current_sum += int_value;
            }
            Err(_) => {
                calorie_sums.push(current_sum);
                current_sum = 0;
            }
        }
    }

    calorie_sums
}
