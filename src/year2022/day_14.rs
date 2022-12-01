use std::io::Result;

pub fn main() -> Result<()> {
    let input = super::super::utils::read_file("src/year2022/day_14/input.txt");
    println!("input: {:?}", input);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_14_test.txt";

    #[test]
    fn test_part_1() {
        let input = read_file(TEST_INPUT);
        // assert_eq!(part_1(&input), (... ));
    }

    #[test]
    fn test_part_2() {
        let input = read_file(TEST_INPUT);
        // assert_eq!(part_2(&input), ( ... ))
    }
}
