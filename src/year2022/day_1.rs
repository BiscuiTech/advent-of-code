use std::fs;

const FILE_PATH: &str = "src/year2022/data/day_1.txt";

pub fn main() {
    let input = read_file(FILE_PATH);
    println!("Part 1: {:?}", part_1(&input));
    let input: &[Vec<u32>] = &input;
    println!("Part 2: {:?}", part_2(input));
}

fn part_1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|vec| vec.iter().sum()).max().unwrap()
}

fn part_2(input: &[Vec<u32>]) -> u32 {
    let mut sums: Vec<u32> = input.iter().map(|x| x.iter().sum()).collect();
    sums.sort();
    sums.as_slice()[sums.len() - 3..].to_vec().iter().sum()
}

fn read_file(path: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    contents
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_1_test.txt";

    #[test]
    fn test_part_1() {
        let input = read_file(TEST_INPUT);
        assert_eq!(part_1(&input), (24000));
    }

    #[test]
    fn test_part_2() {
        let input = read_file(TEST_INPUT);
        assert_eq!(part_2(&input), (45000))
    }
}
