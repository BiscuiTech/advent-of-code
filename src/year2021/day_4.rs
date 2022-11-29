use hashbrown::HashMap;
use std::fs;

pub fn main() {}

// fn get_file_content(path: &str) -> (&str, &str) {
//     let (nums, boards) = fs::read_to_string(path)
//         .unwrap()
//         .split_once("\n\n")
//         .unwrap();
//     (nums, boards)
// }

// fn part_1(path: &str) -> u32 {
//     let mut boards: Vec<(HashMap<u8, usize>, u32)> = boards
//         .split("\n\n")
//         .map(|b| {
//             (
//                 b.split_ascii_whitespace()
//                     .enumerate()
//                     .map(|(i, n)| (n.parse().unwrap(), i))
//                     .collect(),
//                 0,
//             )
//         })
//         .collect();
//     (nums, boards)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_1() {
//         assert_eq!(part_1("src/data/day4_test.txt"), 4512);
//     }

//     // #[test]
//     // fn test_part_2() {
//     //     assert_eq!(part_2(), 0);
//     // }
// }
