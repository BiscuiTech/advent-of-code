use std::{collections::BTreeMap, fs};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

pub fn main() {
    let input = fs::read_to_string("src/year2022/data/day_7.txt").unwrap();
    println!("Part 1: {:?}", part_1(input.as_str()));
    println!("Part 2: {:?}", part_2(input.as_str()));
}

fn part_1(input: &str) -> String {
    let cmds = commands(input).unwrap().1;

    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

fn part_2(input: &str) -> String {
    let cmds = commands(input).unwrap().1;
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = sizes.get(&vec![""]).unwrap();

    let current_free_space = total_size - used_space;
    let need_to_free_at_least = needed_space - current_free_space;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    valid_dirs.sort();
    valid_dirs.get(0).unwrap().to_string()
}

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}
#[derive(Debug)]
enum Files<'a> {
    File { size: u32 },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;
    Ok((input, Files::File { size }))
}
fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}
fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}
fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}
fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}

fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    command: &'a Operation,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        Operation::Cd(Cd::Root) => {
            context.push("");
        }
        Operation::Cd(Cd::Up) => {
            context.pop();
        }
        Operation::Cd(Cd::Down(name)) => {
            context.push(name);
        }
        Operation::Ls(files) => {
            let sum = files
                .iter()
                .filter_map(|file| {
                    if let Files::File { size, .. } = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>();

            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            }
        }
    };
    (context, sizes)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_7_test.txt";

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();
        assert_eq!(part_1(input.as_str()), "95437");
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();
        assert_eq!(part_2(input.as_str()), "24933642");
    }
}
