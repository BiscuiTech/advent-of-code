fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

const STRING_NUMBERS: [(&str, u8); 9] = [
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

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut payload = line.chars().enumerate().filter_map(|(index, char)| {
                if let Some(digit) = char.to_digit(10) {
                    Some(digit)
                } else {
                    let sub_str = &line[index..];
                    STRING_NUMBERS.iter().find_map(|(digit_as_string, digit)| {
                        sub_str
                            .starts_with(digit_as_string)
                            .then_some(*digit as u32)
                    })
                }
            });
            let first = payload.next().expect("Not found");
            let last = payload.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
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
