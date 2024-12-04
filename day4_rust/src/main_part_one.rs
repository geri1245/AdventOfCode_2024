use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use glam::IVec2;

const WORD_TO_LOOK_FOR: &str = "XMAS";

fn matrix_at(matrix: &Vec<&str>, coords: &IVec2) -> char {
    matrix[coords.x as usize].as_bytes()[coords.y as usize] as char
}

fn check_bounds(matrix: &Vec<&str>, coords: &IVec2) -> bool {
    coords.min_element() >= 0
        && (coords.x as usize) < matrix.len()
        && (coords.y as usize) < matrix[coords.x as usize].as_bytes().len()
}

fn xmas_count_in_all_directions(
    letter_matrix: &Vec<&str>,
    directions: &Vec<IVec2>,
    coords: &IVec2,
) -> u32 {
    let mut xmas_count = 0;
    for direction in directions {
        // If the last position is still in the vector, then all of the previous ones are
        // also safe
        let final_position = coords + 3 * direction;
        if !check_bounds(letter_matrix, &final_position) {
            continue;
        }

        let mut is_word_correct = true;

        for i in 1..4usize {
            let letter_to_look_for = WORD_TO_LOOK_FOR.as_bytes()[i] as char;
            if matrix_at(letter_matrix, &(coords + (i as i32) * direction)) != letter_to_look_for {
                is_word_correct = false;
                break;
            }
        }
        if is_word_correct {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn main() -> Result<(), io::Error> {
    let abs_path = Path::new("input.txt");

    let mut input_file = File::open(abs_path)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    let mut valid_directions = Vec::new();
    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            if i == 0 && j == 0 {
                continue;
            }
            valid_directions.push(IVec2::new(i, j));
        }
    }

    let mut letter_matrix = Vec::new();
    for line in content.lines() {
        letter_matrix.push(line);
    }

    let mut xmas_count = 0u32;
    for i in 0..letter_matrix.len() {
        for j in 0..letter_matrix.len() {
            let coords = IVec2::new(i as i32, j as i32);
            if letter_matrix[i].as_bytes()[j] as char == 'X' {
                xmas_count +=
                    xmas_count_in_all_directions(&letter_matrix, &valid_directions, &coords);
            }
        }
    }

    println!("XMAS count = {xmas_count}");

    Ok(())
}
