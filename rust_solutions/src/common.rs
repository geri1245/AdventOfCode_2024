use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn read_input(path_str: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path_str);

    let mut input_file = File::open(path)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    Ok(content
        .lines()
        .map(|str_slice| str_slice.to_owned())
        .collect::<Vec<String>>())
}
