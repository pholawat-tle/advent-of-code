use std::fs;

pub fn read_txt(file_name: &str) -> String {
    let year_from_file_name = file_name
        .split('_')
        .nth(1)
        .expect("Error parsing year from file name");

    let actual_file_path = format!("input/{}/{}.txt", year_from_file_name, file_name);

    let error_message = format!("Error reading file: {}", file_name);
    fs::read_to_string(actual_file_path).expect(&error_message)
}
