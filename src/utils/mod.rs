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

pub fn delete_last_action() {
    let path = Path::new("last_action.txt");
    fs::remove_file(path).unwrap();
}
