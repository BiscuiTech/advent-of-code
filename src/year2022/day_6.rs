use std::fs;

pub fn main() {
    let input = read_file("src/year2022/data/day_6.txt");
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

fn part_1(input: &String) -> usize {
    input
        .as_bytes()
        .windows(4)
        .position(|window| {
            !window
                .iter()
                .enumerate()
                .any(|(i, character)| window[..i].contains(character))
        })
        .unwrap()
        + 4
}

fn part_2(input: &String) -> usize {
    input
        .as_bytes()
        .windows(14)
        .position(|window| {
            !window
                .iter()
                .enumerate()
                .any(|(i, character)| window[..i].contains(character))
        })
        .unwrap()
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_1 = part_1(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        assert_eq!(test_1, 7);
        let test_2 = part_1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        assert_eq!(test_2, 5);
        let test_3 = part_1(&"nppdvjthqldpwncqszvftbrmjlhg".to_string());
        assert_eq!(test_3, 6);
        let test_4 = part_1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());
        assert_eq!(test_4, 10);
        let test_5 = part_1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());
        assert_eq!(test_5, 11);
    }

    #[test]
    fn test_part_2() {
        let test_1 = part_2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        assert_eq!(test_1, 19);
        let test_2 = part_2(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        assert_eq!(test_2, 23);
        let test_3 = part_2(&"nppdvjthqldpwncqszvftbrmjlhg".to_string());
        assert_eq!(test_3, 23);
        let test_4 = part_2(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());
        assert_eq!(test_4, 29);
        let test_5 = part_2(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());
        assert_eq!(test_5, 26);
    }
}
