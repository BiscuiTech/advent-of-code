use regex::*;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let color_reg =
        Regex::new(r"(((?<red>\d+)\ red)|((?<green>\d+)\ green)|((?<blue>\d+)\ blue))").unwrap();

    input
        .lines()
        .map(|line| {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            color_reg.captures_iter(line).for_each(|cap| {
                if let Some(r) = cap.name("red") {
                    let r = r.as_str().parse::<u32>().unwrap_or(0);
                    if red < r {
                        red = r;
                    }
                }
                if let Some(b) = cap.name("blue") {
                    let b = b.as_str().parse::<u32>().unwrap_or(0);
                    if blue < b {
                        blue = b;
                    }
                }
                if let Some(g) = cap.name("green") {
                    let g = g.as_str().parse::<u32>().unwrap_or(0);
                    if green < g {
                        green = g;
                    }
                }
            });
            return red * green * blue;
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286)
    }
}
