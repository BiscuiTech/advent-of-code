pub fn main() {
    println!("{:?}", part_1(read_file()));
    println!("{:?}", part_2(read_file()));
}

fn read_file() -> &'static [u8; 9999] {
    include_bytes!("./data/day_2.txt")
}

fn part_1(input: &[u8]) -> i16 {
    input
        .split(|b| *b == b'\n')
        .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16))
        .map(|(a, b)| 1 + b + 3 * ((1 + b - a).rem_euclid(3)))
        .sum::<i16>()
}

fn part_2(input: &[u8]) -> i16 {
    input
        .split(|b| *b == b'\n')
        .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16))
        .map(|(a, b)| 1 + b * 3 + (2 + a + b) % 3)
        .sum::<i16>()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn read_test_file() -> &'static [u8; 11] {
        include_bytes!("./data/day_2_test.txt")
    }

    #[test]
    fn test_part_1() {
        let payload = part_1(read_test_file());
        assert_eq!(payload, (15));
    }

    #[test]
    fn test_part_2() {
        let payload = part_2(read_test_file());
        assert_eq!(payload, (12));
    }
}
