use std::fs;

const FILE_PATH: &str = "src/year2022/data/day_1.txt";

pub fn main() {
    let input = read_file(FILE_PATH);
    println!("input: {:?}", part_1(input.clone()));
    println!("input: {:?}", part_2(input));
}

fn part_1(input: Vec<Vec<u32>>) -> (u32, u32) {
    // sum each inner vec
    let mut result = Vec::new();
    for vec in input {
        let mut sum = 0;
        for num in vec {
            sum += num;
        }
        result.push(sum);
    }
    // get index of max sum
    let mut max = 0;
    let mut max_index = 0;
    for (index, sum) in result.iter().enumerate() {
        if *sum > max {
            max = *sum;
            max_index = index + 1;
        }
    }
    // return max sum vec
    (max, max_index as u32)
}

fn part_2(input: Vec<Vec<u32>>) -> u32 {
    let mut sums: Vec<u32> = input.iter().map(|x| x.iter().sum()).collect();
    sums.sort();
    sums.as_slice()[sums.len() - 3..].to_vec().iter().sum()
}

fn read_file(path: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    // seperate empty lines into a new vector
    let mut elves: Vec<Vec<u32>> = vec![];
    let raw: Vec<&str> = contents.split("\n\n").collect();
    for group in raw {
        // let mut group_vec: Vec<u32> = vec![];
        elves.push(
            group
                .lines()
                .map(|s| s.to_string().parse::<u32>().unwrap())
                .collect(),
        );
    }
    elves
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_1_test.txt";

    #[test]
    fn test_part_1() {
        let input = read_file(TEST_INPUT);
        assert_eq!(part_1(input), (24000, 4));
    }

    #[test]
    fn test_part_2() {
        let input = read_file(TEST_INPUT);
        assert_eq!(part_2(input), (45000))
    }
}
