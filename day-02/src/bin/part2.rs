struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("./part2_data.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> u32 {
    input.lines().map(parse_game).sum()
}

fn parse_game(input: &str) -> u32 {
    let parts: Vec<&str> = input.split(':').collect();

    let sets: Vec<&str> = parts[1].split(';').collect();
    let sets: Vec<Set> = sets.iter().map(|&set| parse_set(set)).collect();

    let mut max_set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    for set in sets {
        max_set.red = max_set.red.max(set.red);
        max_set.green = max_set.green.max(set.green);
        max_set.blue = max_set.blue.max(set.blue);
    }
    max_set.red * max_set.green * max_set.blue
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
        let input = include_str!("./part2_test_data.txt");
        let actual_result = run(input);
        let expected_result = 2286;
        assert_eq!(actual_result, expected_result);
    }
}
