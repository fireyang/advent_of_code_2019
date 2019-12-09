// use std::fs::File;
use std::fs;

pub fn parse_from_file(path: &str) -> Result<Vec<String>, &'static str> {
    let read_str = fs::read_to_string(path).expect("file not found");
    let contents: Vec<String> = read_str
        .trim()
        .lines()
        .map(|x| x.trim())
        .filter(|&x| x.len() != 0)
        .map(|x| x.to_string())
        .collect();
    Ok(contents)
}

pub fn read_from_file(path: &str) -> Result<String, &'static str> {
    let mut contents = fs::read_to_string(path).expect("file not found");
    contents = contents.trim().to_string();
    Ok(contents)
}

pub fn parse_from_str(val: &str) -> Vec<String> {
    let str_vec: Vec<String> = val
        .lines()
        .map(|x| x.trim())
        .filter(|&x| x.len() != 0)
        .map(|x| x.to_string())
        .collect();
    str_vec
}
