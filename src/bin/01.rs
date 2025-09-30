use std::sync::LazyLock;

use regex::Regex;

advent_of_code::solution!(1);

// fn match_helper(string: &str) -> bool {
//     RE.is_match(string)
// }

pub fn part_one(input: &str) -> Option<u64> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?<left>\d+)(\s+)(?<right>\d+)").unwrap());
    let mut _distances: Vec<u64> = vec![];
    let result: u64 = input.split("\r\n").map(|s| {
        println!("{:?}", s);
        let caps = RE.captures(s).unwrap();
        let left = caps.get("left");
        let right = caps.get("right");
    });
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
