use std::ops::Range;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

const LINE_LENGTH: u32 = 140;

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    input.lines().enumerate().for_each(|(line_index, line)| {
        let mut number_start_index: Option<u32> = None;
        let mut number_end_index: Option<u32> = None;

        line.chars().enumerate().for_each(|(char_index, char)| {
            // check if char is integer
            if char.is_digit(10) {
                if number_start_index.is_none() {
                    number_start_index = Some(char_index as u32);
                    return;
                } else {
                    return;
                }
            } else if number_start_index.is_some() {
                // todo: unbounds check
                number_end_index = Some(char_index as u32 - 1);
                if validation(
                    &lines,
                    line_index,
                    (number_start_index.unwrap(), number_end_index.unwrap()),
                ) {
                    let num = line
                        .get(Range {
                            start: number_start_index.unwrap() as usize,
                            end: number_end_index.unwrap() as usize + 1,
                        })
                        .unwrap();
                    // dbg!(num);

                    let num = num.parse::<u32>().unwrap();
                    sum += num;
                    // println!("✅ Num {}", &num);
                    // dbg!(
                    //     line_index,
                    //     char_index,
                    //     number_start_index,
                    //     number_end_index,
                    //     num,
                    //     sum
                    // );
                }
                number_start_index = None;
                number_end_index = None;
            };
        });
        // dbg!(sum);
    });
    sum
}

fn validation(lines: &Vec<&str>, line_index: usize, number: (u32, u32)) -> bool {
    let line_above = line_index > 0;
    let line_below = line_index + 1 <= lines.len() - 1;
    let line_left = number.0 > 0;
    let line_right = number.1 + 1 <= LINE_LENGTH;
    // dbg!(line_index, number, lines.len(), line_below);
    if line_above {
        if lines[line_index - 1]
            .get(Range {
                start: number.0 as usize,
                end: number.1 as usize + 1,
            })
            .unwrap()
            .chars()
            .into_iter()
            .filter_map(|x| Some(is_symbol(&x)))
            .collect::<Vec<bool>>()
            .contains(&true)
        {
            return true;
        }
    }
    if line_below {
        if lines[line_index + 1]
            .get(Range {
                start: number.0 as usize,
                end: number.1 as usize + 1,
            })
            .unwrap()
            .chars()
            .into_iter()
            .filter_map(|x| Some(is_symbol(&x)))
            .collect::<Vec<bool>>()
            .contains(&true)
        {
            return true;
        }
    }
    if line_left {
        if is_symbol(
            &lines[line_index]
                .chars()
                .nth(number.0 as usize - 1)
                .unwrap(),
        ) {
            return true;
        }
        if line_above {
            if is_symbol(
                &lines[line_index - 1]
                    .chars()
                    .nth(number.0 as usize - 1)
                    .unwrap(),
            ) {
                return true;
            }
        }
        if line_below {
            if is_symbol(
                &lines[line_index + 1]
                    .chars()
                    .nth(number.0 as usize - 1)
                    .unwrap(),
            ) {
                return true;
            }
        }
    }
    if line_right {
        // dbg!(line_index, number);
        if is_symbol(
            &lines[line_index]
                .chars()
                .nth(number.1 as usize + 1)
                .unwrap(),
        ) {
            return true;
        }
        if line_above {
            if is_symbol(
                &lines[line_index - 1]
                    .chars()
                    .nth(number.1 as usize + 1)
                    .unwrap(),
            ) {
                return true;
            }
        }
        if line_below {
            if is_symbol(
                &lines[line_index + 1]
                    .chars()
                    .nth(number.1 as usize + 1)
                    .unwrap(),
            ) {
                return true;
            }
        }
    }
    return false;
}

fn is_symbol(x: &char) -> bool {
    !x.is_digit(10) && *x != '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361)
    }
}
