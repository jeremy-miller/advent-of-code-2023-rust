use std::collections::HashMap;

const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("./part2_data.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> usize {
    input.lines().map(parse_line).sum()
}

fn parse_line(input: &str) -> usize {
    let mut first_digit_map: HashMap<usize, usize> = HashMap::new();
    let mut last_digit_map: HashMap<usize, usize> = HashMap::new();
    for (index, digit) in DIGITS.iter().enumerate() {
        if let Some(i) = input.find(digit) {
            first_digit_map.insert(index + 1, i);
        }
        if let Some(i) = input.rfind(digit) {
            last_digit_map.insert(index + 1, i);
        }
    }
    for (index, digit_word) in DIGIT_WORDS.iter().enumerate() {
        if let Some(i) = input.find(digit_word) {
            let current_index = first_digit_map.entry(index + 1).or_insert(input.len() - 1);
            if i < *current_index {
                *current_index = i;
            }
        }
        if let Some(i) = input.rfind(digit_word) {
            let current_index = last_digit_map.entry(index + 1).or_insert(0);
            if i > *current_index {
                *current_index = i;
            }
        }
    }
    let first_digit = first_digit_map
        .iter()
        .min_by_key(|(_, value)| *value)
        .expect("error finding min")
        .0;
    let second_digit = last_digit_map
        .iter()
        .max_by_key(|(_, value)| *value)
        .expect("error finding max")
        .0;
    format!("{first_digit}{second_digit}")
        .parse::<usize>()
        .expect("error parsing number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_official_test_data() {
        let input = include_str!("./part2_test_data.txt");
        let actual_result = run(input);
        let expected_result = 281;
        assert_eq!(actual_result, expected_result);
    }
}
