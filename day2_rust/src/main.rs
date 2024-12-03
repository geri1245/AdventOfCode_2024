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

fn is_data_safe(levels: &[i32]) -> bool {
    let level_count = levels.len();

    // If we have at most 2 items, then we can remove one of them and the condition will be true no matter what
    if level_count < 3 {
        return true;
    }

    // Calculate the difference between neighboring items, we will make something out of them later
    let mut neighbor_differences = levels
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, item)| item - levels[index - 1])
        .collect::<Vec<_>>();

    // These will contain the indices where the neighbors' difference is positive, negative or zero respectively
    let mut pos_diffs = vec![];
    let mut neg_diffs = vec![];
    let mut no_diffs = vec![];
    // Look through the first 3 items. Based on that we can decide if the sequence is increasing or not.
    // If we can't decide, that means the sequence is not correct
    for (index, difference) in neighbor_differences.iter().enumerate().take(4) {
        if *difference > 0 {
            pos_diffs.push(index);
        } else if *difference == 0 {
            no_diffs.push(index);
        } else if *difference < 0 {
            neg_diffs.push(index);
        }
    }

    let is_increasing = pos_diffs.len() > neg_diffs.len();
    let mut is_correction_available = true;
    if !is_difference_correct(neighbor_differences[0], is_increasing) {
        if !is_difference_correct(neighbor_differences[1], is_increasing) {
            neighbor_differences[1] += neighbor_differences[0];
        }
        is_correction_available = false;
    }

    if !is_difference_correct(
        neighbor_differences[neighbor_differences.len() - 1],
        is_increasing,
    ) {
        if !is_correction_available {
            return false;
        }
        if !is_difference_correct(
            neighbor_differences[neighbor_differences.len() - 2],
            is_increasing,
        ) {
            let neighbor_diff_count = neighbor_differences.len();
            neighbor_differences[neighbor_diff_count - 2] +=
                neighbor_differences[neighbor_diff_count - 1];
        }
        is_correction_available = false;
    }

    for i in 1..(neighbor_differences.len() - 1) {
        if !is_difference_correct(neighbor_differences[i], is_increasing) {
            if is_difference_correct(
                neighbor_differences[i] + neighbor_differences[i + 1],
                is_increasing,
            ) && is_correction_available
            {
                is_correction_available = false;
            } else {
                return false;
            }
        }
    }

    true
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
            println!("{line} is NOT correct")
        }
        lines_processed += 1;
    }

    println!("Safe level count: {safe_level_count} with {lines_processed} lines processed");
    Ok(())
}
