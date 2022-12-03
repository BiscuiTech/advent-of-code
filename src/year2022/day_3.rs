pub fn main() {
    println!("Part 1: {:?}", part_1("src/year2022/data/day_3.txt"));
    println!("Part 2: {:?}", part_2("src/year2022/data/day_3.txt"));
}

fn parse_into_bytes(input: &str) -> Vec<u16> {
    input
        .chars()
        .map(|x| match x.is_uppercase() {
            false => (x as u16 + 1) - b'a' as u16,
            true => ((x as u16 + 1) - b'A' as u16) + 26,
        })
        .collect()
}

fn part_1(path: &str) -> u16 {
    let input = crate::utils::read_file(path);
    // turn the input into a Vec<(Vec<u16>, Vec<u16>)>
    let input = input
        .iter()
        .map(|l| {
            let mut left = parse_into_bytes(l);
            let right = left.split_off(l.len() / 2);
            (left, right)
        })
        .collect::<Vec<(Vec<u16>, Vec<u16>)>>();

    let mut sum = 0;
    input.iter().for_each(|x| {
        let left = &x.0;
        let right = &x.1;
        // take the identical item of the two vectors, and add it to sum
        let index = left.iter().position(|&x| right.contains(&x));
        if let Some(x) = index {
            sum += left[x]
        }
    });
    sum
}

fn part_2(path: &str) -> u16 {
    let input = crate::utils::read_file(path);
    let input = input
        .iter()
        .map(|l| parse_into_bytes(l))
        .collect::<Vec<Vec<u16>>>();
    // make groups of 3s
    let mut sum: u16 = 0;
    let mut base = 0;
    for i in 0..input.len() / 3 {
        let index = i + base;
        let first = &input[index];
        let second = &input[index + 1];
        let third = &input[index + 2];
        // get the longest vector
        /* let longest =  */
        if first.len() > second.len() {
            if first.len() > third.len() {
                // first
                println!("first->first");
                calculate_part_2(first, second, third, &mut sum)
            } else {
                // third
                println!("first->third");
                calculate_part_2(third, first, second, &mut sum)
            }
        } else if second.len() > third.len() {
            // second
            println!("second -> second");
            calculate_part_2(second, first, third, &mut sum)
        } else {
            // third
            println!("third -> third");
            calculate_part_2(third, first, second, &mut sum)
        };
        base += 2;
    }
    // find the sum of the groups
    sum
}

fn calculate_part_2(longest: &[u16], first: &[u16], second: &[u16], sum: &mut u16) {
    let mut found = false;
    longest.iter().for_each(|x| {
        // check if the other two vectors contain the same item
        if !found && first.contains(x) && second.contains(x) {
            found = true;
            // if so, add it to sum
            println!(
                "X is: {:?} - {:?}",
                x,
                char::from_u32(*x as u32 + 96).unwrap()
            );
            *sum += x;
        }
    });
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
