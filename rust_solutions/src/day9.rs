use std::io;

fn next_free_space_from(from: usize, vec: &Vec<i32>) -> usize {
    for i in from..vec.len() {
        if vec[i] == -1 {
            return i;
        }
    }

    vec.len()
}

fn next_contiguous_free_space_from(from: usize, required_space: usize, vec: &Vec<i32>) -> usize {
    let mut current_position = from;
    loop {
        let mut has_enough_space = true;
        for i in current_position..(current_position + required_space) {
            if i >= vec.len() {
                return vec.len();
            }

            if vec[i] != -1 {
                has_enough_space = false;
                break;
            }
        }
        if has_enough_space {
            return current_position;
        } else {
            current_position = next_free_space_from(current_position + 1, vec);
            if current_position == vec.len() {
                break;
            }
        }
    }

    vec.len()
}

fn same_character_count_in_reverse(from: usize, vec: &Vec<i32>) -> usize {
    let first_element = vec[from];
    vec.iter()
        .rev()
        .skip(vec.len() - from - 1)
        .take_while(|num| **num == first_element)
        .count()
}

pub fn _day9_part1(input_lines: &[String]) -> io::Result<u64> {
    let input = &input_lines[0];

    let mut is_file = true;
    let mut current_file_id = 0;
    let mut expanded_filesystem = Vec::new();

    // Expand the short notation. -1 means empty space
    for byte in input.as_bytes() {
        let char = *byte as char;
        let num = char.to_string().parse::<usize>().unwrap();
        if is_file {
            expanded_filesystem.extend_from_slice(&vec![current_file_id; num]);
            current_file_id += 1;
        } else {
            expanded_filesystem.extend_from_slice(&vec![-1; num]);
        }
        is_file = !is_file;
    }

    let mut current_location = (expanded_filesystem.len() as i64) - 1;
    while current_location >= 0 {
        if expanded_filesystem[current_location as usize] != -1 {
            let count =
                same_character_count_in_reverse(current_location as usize, &expanded_filesystem);
            let free_space_index = next_contiguous_free_space_from(0, count, &expanded_filesystem);

            // Check if the space we have found is earlier than the chunk we would like to move
            if free_space_index != expanded_filesystem.len()
                && free_space_index + count < (current_location as usize)
            {
                // Copy the data to the correct spot
                for i_char in 0..count {
                    expanded_filesystem[free_space_index + i_char] =
                        expanded_filesystem[(current_location - (i_char as i64)) as usize];
                    expanded_filesystem[(current_location - (i_char as i64)) as usize] = -1;
                }
            }
            current_location -= count as i64;
        } else {
            current_location -= 1;
        }
    }

    let mut chechsum = 0;
    for (index, file_id) in expanded_filesystem.iter().enumerate() {
        chechsum += if *file_id == -1 {
            0
        } else {
            index * (*file_id as usize)
        };
    }

    Ok(chechsum as u64)
}
