use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    *,
};
use std::fs;

#[derive(Debug)]
struct Move {
    number: u32,
    from: u32,
    to: u32,
}

pub fn main() {
    let input = read_file("src/year2022/data/day_5.txt");

    println!("Part 1: {:?}", part_1(input.as_str()));
    println!("Part 2: {:?}", part_2(input.as_str()));
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(contents) => contents,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}
fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

fn move_crate(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((
        input,
        Move {
            number,
            from: from - 1,
            to: to - 1,
        },
    ))
}
fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, move_crate)(input)?;

    let mut crates_vertical: Vec<Vec<Option<&str>>> = vec![];
    for _ in 0..=crates_horizontal.len() {
        crates_vertical.push(vec![]);
    }
    for vec in crates_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            crates_vertical[i].push(*c)
        }
    }
    let final_crates: Vec<Vec<&str>> = crates_vertical
        .iter()
        .map(|vec| vec.iter().filter_map(|v| *v).collect())
        .collect();

    Ok((input, (final_crates, moves)))
}

fn part_1(input: &str) -> String {
    let (_, (mut crate_stacks, moves)) = crates(input).unwrap();
    for Move { number, from, to } in moves.iter() {
        let len = crate_stacks[*from as usize].len();
        let drained = crate_stacks[*from as usize]
            .drain((len - *number as usize)..)
            .rev()
            .collect::<Vec<&str>>();
        for c in drained.iter() {
            crate_stacks[*to as usize].push(c);
        }
    }
    let result: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    result
}

fn part_2(input: &str) -> String {
    let (_, (mut crate_stacks, moves)) = crates(input).unwrap();
    for Move { number, from, to } in moves.iter() {
        let len = crate_stacks[*from as usize].len();
        let drained = crate_stacks[*from as usize]
            .drain((len - *number as usize)..)
            .collect::<Vec<&str>>();
        for c in drained.iter() {
            crate_stacks[*to as usize].push(c);
        }
    }

    let result: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_5_test.txt";

    #[test]
    fn test_part_1() {
        let input = read_file(TEST_INPUT);
        let result = part_1(input.as_str());
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let input = read_file(TEST_INPUT);
        let result = part_2(input.as_str());
        // assert_eq!(result, "MCD");
    }
}
