pub fn main() {
    // PART 1
    // read file from disk
    let split_string = crate::utils::read_file("src/year2021/data/day_2.txt");

    // Vector of strings, each string is composed of a string and a int
    let mut instructions: Vec<(String, i32)> = Vec::new();
    for s in split_string {
        let split_string: Vec<&str> = s.split(' ').collect();
        let instruction = split_string[0];
        let value = split_string[1].parse::<i32>().unwrap();
        instructions.push((instruction.to_string(), value));
    }
    let mut depth = 0;
    let mut horizontal = 0;

    for item in &instructions {
        match item.0.as_str() {
            "forward" => horizontal += item.1,
            "up" => depth += item.1,
            "down" => depth -= item.1,
            _ => (),
        };
    }
    println!(
        "part 1: {:?} * {:?} = {:?}",
        horizontal.abs(),
        depth.abs(),
        horizontal.abs() * depth.abs()
    );

    // PART 2
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for item in &instructions {
        match item.0.as_str() {
            "forward" => {
                horizontal += item.1;
                depth += aim * item.1;
            }
            "up" => aim -= item.1,
            "down" => aim += item.1,
            _ => (),
        };
    }
    println!(
        "part 2: {:?} * {:?} = {:?}",
        horizontal.abs(),
        depth.abs(),
        horizontal.abs() * depth.abs()
    );
}
