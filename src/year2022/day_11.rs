use std::fs;
pub fn main() {
    let input = fs::read_to_string("src/year2022/data/day_11.txt").unwrap().as_str();
    println!("Part 1: {:?}", part_1(input));
    println!("Part 2: {:?}", part_2(input));
}

fn part_1(input: &str) {
    todo!()
}

fn part_2(input: &str) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_11_test.txt";

    #[test]
    fn test_part_1() {
        todo!()
        // let input = read_file(TEST_INPUT);
        // assert_eq!(part_1(&input), (... ));
    }

    #[test]
    fn test_part_2() {
        todo!()

        // let input = read_file(TEST_INPUT);
        // assert_eq!(part_2(&input), ( ... ))
    }
}
