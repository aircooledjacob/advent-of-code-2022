pub fn parse_lines_to_char_tuples(raw_lines_as_string: &Vec<String>) -> Vec<(char, char)> {
    raw_lines_as_string
        .iter()
        .map(|line| {
            let split_line = line.split_once(' ').expect("Malformed input");
            (
                split_line.0.chars().collect::<Vec<char>>()[0],
                split_line.1.chars().collect::<Vec<char>>()[0],
            )
        })
        .collect()
}

pub fn calculate_score(moves_tuple_vec: &Vec<(char, char)>) -> u32 {
    let mut score: u32 = 0;
    for (opponent_choice, your_choice) in moves_tuple_vec {
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
    score
}
