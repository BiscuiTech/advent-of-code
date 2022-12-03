use itertools::Itertools;

pub fn main() {
    let path = "src/year2022/data/day_3.txt";
    println!("Part 1: {:?}", part_1(path));
    println!("Part 2: {:?}", part_2(path));
}

fn part_1(path: &str) -> usize {
    let input = crate::utils::read_file(path);
    // turn the input into a Vec<(Vec<u8>, Vec<u8>)>
    let input = input.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
    input
        .iter()
        .map(|l| {
            l[..l.len() / 2]
                .iter()
                .copied()
                .filter(|x| l[l.len() / 2..].contains(x))
                .collect::<Vec<u8>>()
        })
        .map(|x| get_value(x[0]))
        .sum()
}

fn get_value(v: u8) -> usize {
    match v {
        b'a'..=b'z' => v as usize - b'a' as usize + 1,
        b'A'..=b'Z' => v as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}

fn compare(left: &[u8], right: &[u8]) -> Vec<u8> {
    left.iter().copied().filter(|c| right.contains(c)).collect()
}

fn part_2(path: &str) -> usize {
    let input = crate::utils::read_file(path);
    // turn the input into a Vec<(Vec<u8>, Vec<u8>)>
    let input = input.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
    input
        .iter()
        .tuples()
        .map(|(a, b, c)| compare(a, &compare(b, c)))
        .map(|c| get_value(c[0]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/year2022/data/day_3_test.txt"), (157));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/year2022/data/day_3_test.txt"), (70));
    }
}
