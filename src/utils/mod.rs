use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn read_file(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(contents) => contents.lines().map(|s| s.to_string()).collect(),
        Err(e) => panic!("Error reading file: {}", e),
    }
}

pub fn is_last_action_file_present() -> bool {
    let path = Path::new("last_action.txt");
    path.try_exists()
        .expect("Can't check existence of file does_not_exist.txt")
}

pub fn save_last_action(year: u16, day: u32) {
    let path = Path::new("last_action.txt");
    let mut file = File::create(path).unwrap();
    file.write_all(format!("{} {}", year, day).as_bytes())
        .unwrap();
}

pub fn read_last_action() -> (u16, u32) {
    let path = Path::new("last_action.txt");
    let contents = fs::read_to_string(path).unwrap();
    let mut split = contents.split_ascii_whitespace();
    let year = split.next().unwrap().parse::<u16>().unwrap();
    let day = split.next().unwrap().parse::<u32>().unwrap();
    (year, day)
}

pub fn setup() {
    // create this years's folder if not already present
    let year_path = Path::new("src/year2022");
    match year_path.try_exists() {
        Ok(true) => println!("Year 2022 folder already present"),
        Ok(false) => {
            println!("Creating year 2022 folder");
            fs::create_dir(year_path).unwrap();
        }
        Err(e) => panic!("Error creating year 2022 folder: {}", e),
    }
    // create this years's day folders if not already present
    for day in 1..=25 {
        let path = &format!("src/year2022/day_{}.rs", day);
        let day_path = Path::new(path);
        match day_path.try_exists() {
            Ok(true) => println!("day_{}.rs file already present", day),
            Ok(false) => {
                println!("Creating day_{}.rs file", day);
                // populate file with template data
                let mut file = File::create(day_path).unwrap();
                file.write_all(
                    format!(
                        r#"use std::fs;
pub fn main() {{
    let input = fs::read_to_string("src/year2022/data/day_{day_number}.txt").unwrap().as_str();
    println!("Part 1: {{:?}}", part_1(input));
    println!("Part 2: {{:?}}", part_2(input));
}}

fn part_1(input: &str) {{
    todo!()
}}

fn part_2(input: &str) {{
    todo!()
}}

#[cfg(test)]
mod tests {{
    use super::*;
    const TEST_INPUT: &str = "src/year2022/data/day_{day_number}_test.txt";

    #[test]
    fn test_part_1() {{
        todo!()
        // let input = read_file(TEST_INPUT);
        // assert_eq!(part_1(&input), (... ));
    }}

    #[test]
    fn test_part_2() {{
        todo!()

        // let input = read_file(TEST_INPUT);
        // assert_eq!(part_2(&input), ( ... ))
    }}
}}
"#,
                        day_number = day
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
            Err(e) => panic!("Error creating day_{}.rs file: {}", day, e),
        }
    }
}
