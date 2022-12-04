use itertools::Itertools;

pub fn main() {
    let input = crate::utils::read_file("src/year2022/data/day_4.txt");
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}

fn parse(input: &[String]) -> Vec<Vec<(u8, u8)>> {
    let payload = input
        .iter()
        .map(|line| {
            line.split([',', '-'])
                .map(|s| s.parse::<u8>().unwrap())
                .collect_vec()
        })
        .map(|v| vec![(v[0], v[1]), (v[2], v[3])])
        .collect::<Vec<Vec<(u8, u8)>>>();
    payload
}

fn part_1(input: &[String]) -> usize {
    let count = parse(input)
        .iter()
        .filter(|&v| {
            let a = v[0].0;
            let b = v[0].1;
            let c = v[1].0;
            let d = v[1].1;
            a <= c && b >= d || a >= c && b <= d
        })
        .count();
    count
}

fn part_2(input: &[String]) -> usize {
    let count = parse(input)
        .iter()
        .filter(|&v| {
            let a = v[0].0;
            let b = v[0].1;
            let c = v[1].0;
            let d = v[1].1;
            a <= d && b >= c
        })
        .count();
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_4_test.txt";

    #[test]
    fn test_part_1() {
        let input = crate::utils::read_file(TEST_INPUT);
        assert_eq!(part_1(&input), (2));
    }

    #[test]
    fn test_part_2() {
        let input = crate::utils::read_file(TEST_INPUT);
        assert_eq!(part_2(&input), (4));
    }
}
