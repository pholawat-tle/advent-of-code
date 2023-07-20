use std::fs;

pub fn read_txt(file_name: &str) -> String {
    let error_message = format!("Error reading file: {}", file_name);
    let file_path = format!("input/{}.txt", file_name);
    fs::read_to_string(file_path).expect(&error_message)
}
