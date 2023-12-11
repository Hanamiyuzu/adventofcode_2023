use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait AnyChars {
    fn from_str(s: String) -> Self;
}

impl AnyChars for String {
    fn from_str(s: String) -> Self {
        s
    }
}

impl AnyChars for Vec<char> {
    fn from_str(s: String) -> Self {
        s.chars().collect()
    }
}

pub fn get_lines_from_file<T: AnyChars>(file_path: &str) -> Vec<T> {
    let file = File::open(file_path).unwrap();
    let mut lines = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(T::from_str(line.unwrap()));
    }
    lines
}
