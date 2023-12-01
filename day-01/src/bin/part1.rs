fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let payload: u32 = input
        .lines()
        .map(|row| {
            let payload = row
                .chars()
                .filter(|x| x.is_digit(10))
                .collect::<Vec<char>>();
            let first = payload.first().unwrap();
            let last = payload.last().unwrap();
            format!("{}{}", first, last).parse::<u32>().ok().unwrap()
        })
        .sum();
    dbg!(payload);
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142)
    }
}
