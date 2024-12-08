use std::{fs::File, io::Read};

pub fn get_problem_input(path: &std::path::Path) -> std::io::Result<String> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
