pub fn main() {
    // PART 1
    let split_string = crate::utils::read_file("src/year2021/data/day_1.txt");
    // convert string to int
    let readings: Vec<i32> = split_string
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    // iterate over readings
    let mut total_increases = 0;
    for i in 0..readings.len() {
        if i > 0 && readings[i] > readings[i - 1] {
            total_increases += 1;
        }
    }
    println!("part 1: {:?}", total_increases);

    // PART 2
    // iterate over readings
    let mut total_combined_increases = 0;

    for i in 0..readings.len() {
        // check if there are enough items to compare (4 items)

        if readings.len() - i >= 4 {
            // combine the first 3 items
            let low_combined_readings = readings[i] + readings[i + 1] + readings[i + 2];
            let high_combined_readings = readings[i + 1] + readings[i + 2] + readings[i + 3];
            // check if the high readings are higher than the low readings
            if high_combined_readings > low_combined_readings {
                total_combined_increases += 1;
            }
        } else {
            break;
        }
    }
    println!("part 2: {:?}", total_combined_increases);
}
