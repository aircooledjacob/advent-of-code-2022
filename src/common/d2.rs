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
