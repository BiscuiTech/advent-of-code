use regex::*;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    const MAX_RED: u8 = 12;
    const MAX_BLUE: u8 = 14;
    const MAX_GREEN: u8 = 13;

    let color_reg =
        Regex::new(r"(((?<red>\d+)\ red)|((?<green>\d+)\ green)|((?<blue>\d+)\ blue))").unwrap();
    let game_number_reg = Regex::new(r"Game (?<game_number>\d+):").unwrap();

    input
        .lines()
        .map(|line| {
            let round_result = line
                .split(";")
                .map(|round| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    let _ = color_reg
                        .captures_iter(round)
                        .map(|cap| {
                            if let Some(r) = cap.name("red") {
                                red += r.as_str().parse::<u32>().unwrap_or(0);
                            }
                            if let Some(b) = cap.name("blue") {
                                blue += b.as_str().parse::<u32>().unwrap_or(0);
                            }
                            if let Some(g) = cap.name("green") {
                                green += g.as_str().parse::<u32>().unwrap_or(0);
                            }
                        })
                        .collect::<()>();
                    if red <= MAX_RED as u32 && green <= MAX_GREEN as u32 && blue <= MAX_BLUE as u32
                    {
                        return true;
                    } else {
                        return false;
                    }
                })
                .collect::<Vec<bool>>();
            if round_result.contains(&false) {
                println!("❌ - {}", line);
                return 0;
            }
            println!("✅ {}", line);
            let game_number = game_number_reg
                .captures(line)
                .unwrap()
                .name("game_number")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            return game_number;
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8)
    }
}
