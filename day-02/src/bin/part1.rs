const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32 = 14;

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("./part1_data.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> u32 {
    input.lines().filter_map(parse_game).sum()
}

fn parse_game(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split(':').collect();

    let gave_vec: Vec<&str> = parts[0].split_whitespace().collect();
    let game_id = gave_vec[1].parse::<u32>().expect("error parsing game ID");

    let sets: Vec<&str> = parts[1].split(';').collect();
    let sets: Vec<Set> = sets.iter().map(|&set| parse_set(set)).collect();

    for set in sets {
        if set.red > NUM_RED || set.green > NUM_GREEN || set.blue > NUM_BLUE {
            return None;
        }
    }
    Some(game_id)
}

fn parse_set(input: &str) -> Set {
    let mut set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    let cubes: Vec<&str> = input.split(',').collect();
    for cube in cubes {
        let parts: Vec<&str> = cube.split_whitespace().collect();
        let num = parts[0].parse::<u32>().expect("error parsing cube number");
        match parts[1] {
            "red" => set.red = num,
            "green" => set.green = num,
            "blue" => set.blue = num,
            _ => panic!("unexpected color"),
        }
    }
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_official_test_data() {
        let input = include_str!("./part1_test_data.txt");
        let actual_result = run(input);
        let expected_result = 8;
        assert_eq!(actual_result, expected_result);
    }
}
