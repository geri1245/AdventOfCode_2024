use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use glam::IVec2;

fn matrix_at(matrix: &Vec<&str>, coords: &IVec2) -> char {
    matrix[coords.x as usize].as_bytes()[coords.y as usize] as char
}

fn check_bounds(matrix: &Vec<&str>, coords: &IVec2) -> bool {
    coords.min_element() >= 0
        && (coords.x as usize) < matrix.len()
        && (coords.y as usize) < matrix[coords.x as usize].as_bytes().len()
}

fn get_words_in_x_shape(matrix: &Vec<&str>, top_left: &IVec2) -> Option<[String; 2]> {
    if !check_bounds(matrix, &(top_left + IVec2 { x: 2, y: 2 })) {
        None
    } else {
        let mut words = [String::new(), String::new()];
        for i in 0..3 {
            words[0].push(matrix_at(matrix, &(top_left + i * IVec2::new(1, 1))));
        }
        let top_right = top_left + IVec2::new(0, 2);
        for i in 0..3 {
            words[1].push(matrix_at(matrix, &(top_right + i * IVec2::new(1, -1))));
        }

        Some(words)
    }
}

fn is_top_left_of_x(letter_matrix: &Vec<&str>, coords: &IVec2) -> bool {
    if let Some(words) = get_words_in_x_shape(letter_matrix, coords) {
        (words[0] == "SAM" || words[0] == "MAS") && (words[1] == "SAM" || words[1] == "MAS")
    } else {
        false
    }
}

fn main() -> Result<(), io::Error> {
    let abs_path = Path::new("input.txt");

    let mut input_file = File::open(abs_path)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    let mut letter_matrix = Vec::new();
    for line in content.lines() {
        letter_matrix.push(line);
    }

    let mut x_mas_count = 0u32;
    for i in 0..letter_matrix.len() {
        for j in 0..letter_matrix.len() {
            let coords = IVec2::new(i as i32, j as i32);
            if is_top_left_of_x(&letter_matrix, &coords) {
                x_mas_count += 1;
            }
        }
    }

    println!("XMAS count = {x_mas_count}");

    Ok(())
}
