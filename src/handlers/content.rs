use std::fs;

pub fn read_file(file_nm: String) -> String {
    fs::read_to_string(file_nm).unwrap()
}