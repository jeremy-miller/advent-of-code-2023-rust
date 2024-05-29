use nom::{character::complete, multi};

fn main() {
    let input = include_str!("./part1_data.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> String {
    let (_, calibration_values) =
        multi::separated_list1(complete::newline, parse_line)(input).expect("error parsing text");
    return calibration_values
        .iter()
        .map(|(first, second)| format!("{first}{second}"))
        .map(|value| value.parse::<u32>().expect("error parsing number"))
        .sum::<u32>()
        .to_string();
}

fn parse_line(input: &str) -> nom::IResult<&str, (char, char)> {
    let (input, _) = complete::alpha0(input)?;
    let (input, first) = complete::one_of("0123456789")(input)?;
    let mut second: Option<char> = None;
    let mut remaining_input = input;
    while !remaining_input.is_empty() {
        let (input, _) = complete::alpha0(remaining_input)?;
        if let Ok((input, value)) =
            complete::one_of::<_, _, nom::error::Error<_>>("0123456789")(input)
        {
            second = Some(value);
            remaining_input = input;
        } else {
            remaining_input = input;
            break;
        }
    }
    second.map_or(Ok((remaining_input, (first, first))), |second_value| {
        Ok((remaining_input, (first, second_value)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_official_test_data() {
        let input = include_str!("./part1_test_data.txt");
        let actual_result = run(input);
        let expected_result = 142;
        assert_eq!(actual_result, expected_result.to_string());
    }

    #[test]
    fn multiple_numbers_next_to_eachother() {
        let input = "10\n10a1";
        let actual_result = run(input);
        let expected_result = 21;
        assert_eq!(actual_result, expected_result.to_string());
    }
}
