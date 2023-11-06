use std::{fs, u32};
struct BinaryCounter {
    zero: u32,
    one: u32,
}

const WIDTH: usize = 12;

fn part1() {
    let split_string = crate::utils::read_file("src/year2021/data/day_3.txt");
    // collect the position of the string into a vector
    let mut matrix = vec![String::new(); split_string[0].chars().count()];
    // fill the matrix with the string
    for char_i in 0..split_string[0].chars().count() {
        for vec_i in 0..split_string.len() {
            // push to matrix the char at the right position
            matrix[char_i].push(split_string[vec_i].chars().nth(char_i).unwrap());
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    // calculate the rate of gamma rate by checking the most frequent character

    for binary_str in matrix {
        let mut counter = BinaryCounter { zero: 0, one: 0 };
        for binary in binary_str.chars() {
            match binary {
                '0' => counter.zero += 1,
                '1' => counter.one += 1,
                _ => (),
            }
        }
        if counter.zero > counter.one {
            epsilon_rate.push('1');
            gamma_rate.push('0');
        } else {
            epsilon_rate.push('0');
            gamma_rate.push('1');
        }
    }
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Gamma rate: {}", gamma_rate);
    let epsilon_decimal: u128 = u128::from_str_radix(&epsilon_rate, 2).unwrap();
    let gamma_decimal: u128 = u128::from_str_radix(&gamma_rate, 2).unwrap();

    println!("Epsilon decimal: {}", epsilon_decimal);
    println!("Gamma decimal: {}", gamma_decimal);
    println!("Answer: {}", epsilon_decimal * gamma_decimal);
}

fn get_life_ratings(nums: Vec<u32>, width: usize) -> (u32, u32) {
    todo!()
    // let oxy = (0..width)
    //     .rev()
    //     .scan(nums.clone(), |oxy, i| {
    //         let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
    //         oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
    //         oxy.first().copied()
    //     })
    //     .last()
    //     .unwrap();

    // let co2 = (0..width)
    //     .rev()
    //     .scan(nums, |co2, i| {
    //         let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
    //         co2.drain_filter(|n| (*n & 1 << i > 0) == one);
    //         co2.first().copied()
    //     })
    //     .last()
    //     .unwrap();
    // (oxy, co2)
}

pub fn main() {
    part1();
    let nums = fs::read_to_string("src/year2021/data/day_3.txt")
        .unwrap()
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<u32>>();

    let (oxy, co2) = get_life_ratings(nums, WIDTH);

    println!("{}", oxy * co2);
}
