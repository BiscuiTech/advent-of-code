fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    // dbg!(ouput)
}

fn part2(input: &str) -> u32 {
    let string_numbers = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    string_numbers
        .into_iter()
        .map(|s| input.replace(s.0, s.1.to_string().as_str()));
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281)
    }
}
