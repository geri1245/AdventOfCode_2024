use std::{
    fs::File,
    io::{self, Read},
    path::{absolute, Path},
};

fn is_difference_correct(diff: i32, is_increasing: bool) -> bool {
    if is_increasing {
        diff > 0 && diff < 4
    } else {
        diff < 0 && diff > -4
    }
}

// Extracts the elements from "array" at indices "indices" into a new Vec
fn extract_elements_from_array(array: &[i32], indices: &[usize]) -> Vec<i32> {
    let mut extracted_items = Vec::new();
    for index in indices {
        extracted_items.push(array[*index]);
    }

    extracted_items
}

fn get_all_arrays_with_one_element_missing(array: &[i32]) -> Vec<Vec<i32>> {
    let all_indices = (0..array.len()).collect::<Vec<_>>();
    let mut all_combiniations = Vec::new();
    for i in 0..array.len() {
        let mut vec_with_one_element_removed = all_indices.clone();
        vec_with_one_element_removed.remove(i);

        all_combiniations.push(extract_elements_from_array(
            array,
            &vec_with_one_element_removed,
        ));
    }

    all_combiniations
}

fn is_sequence_all_same_sign(seq: &[i32], check_positive: bool) -> bool {
    seq.iter()
        .all(|elem| is_difference_correct(*elem, check_positive))
}

fn is_data_safe(levels: &[i32]) -> bool {
    let level_count = levels.len();

    // If we have at most 2 items, then we can remove one of them and the condition will be true no matter what
    if level_count < 3 {
        return true;
    }

    // Calculate the difference between neighboring items, we will make something out of them later
    let neighbor_differences = levels
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, item)| item - levels[index - 1])
        .collect::<Vec<_>>();

    let is_increasing = neighbor_differences[1] > neighbor_differences[0];
    if is_sequence_all_same_sign(&neighbor_differences, is_increasing) {
        true
    } else {
        let all_arays_with_one_element_missing =
            get_all_arrays_with_one_element_missing(&neighbor_differences);
        all_arays_with_one_element_missing
            .iter()
            .any(|sequence| is_sequence_all_same_sign(&sequence, is_increasing))
    }
}

fn main() -> Result<(), io::Error> {
    let abs_path = absolute(Path::new("input.txt"))?;

    let mut input_file = File::open(abs_path)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    let mut safe_level_count = 0;
    let mut lines_processed = 0;
    for line in content.lines() {
        let levels = line
            .split(' ')
            .map(|digit_str| digit_str.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_data_safe(&levels) {
            safe_level_count += 1;
            println!("{line} is correct!")
        } else {
            println!("{line} is NOT correct!")
        }
        lines_processed += 1;
    }

    println!("Safe level count: {safe_level_count} with {lines_processed} lines processed");
    Ok(())
}
