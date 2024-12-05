use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    path::Path,
};

fn do_numbers_match_rules(rules: &HashMap<u32, Vec<u32>>, numbers: &Vec<u32>) -> bool {
    numbers
        .iter()
        .take(numbers.len() - 1)
        .enumerate()
        .all(|(index, number)| {
            let empty_vec = vec![];
            let numbers_following_current_number = rules.get(&number).unwrap_or(&empty_vec);
            ((index + 1)..numbers.len())
                .into_iter()
                .all(|index| numbers_following_current_number.contains(&numbers[index]))
        })
}

fn get_vec_without_number(vec: &Vec<u32>, number_to_exclude: u32) -> Vec<u32> {
    vec.clone()
        .into_iter()
        .filter(|&elem| elem != number_to_exclude)
        .collect::<Vec<u32>>()
}

fn num_of_numbers_contained_in_other(numbers: &Vec<u32>, other: &Vec<u32>) -> usize {
    numbers
        .iter()
        .filter(|number| other.contains(number))
        .count()
}

fn order_numbers_according_to_rules(rules: &HashMap<u32, Vec<u32>>, numbers: &Vec<u32>) -> u32 {
    let num_count = numbers.len();
    let mut ordered_numbers = vec![0; num_count];
    for number in numbers {
        if let Some(following_numbers) = rules.get(&number) {
            let remaining_numbers = get_vec_without_number(numbers, *number);
            let nums_contained =
                num_of_numbers_contained_in_other(&remaining_numbers, following_numbers);
            let result_index = num_count - 1 - nums_contained;
            ordered_numbers[result_index] = *number;
        }
    }
    ordered_numbers[num_count / 2]
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");

    let mut input_file = File::open(path)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    let mut numbers_after_this_number: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut number_arrays = Vec::new();
    let mut is_reading_rules = true;
    for line in content.lines() {
        if line.is_empty() {
            is_reading_rules = false;
            continue;
        }

        if is_reading_rules {
            let numbers = line
                .split('|')
                .map(|str| str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let key = numbers[0];
            let value = numbers[1];

            if let Some(array) = numbers_after_this_number.get_mut(&key) {
                array.push(value);
            } else {
                numbers_after_this_number.insert(key, vec![value]);
            }
        } else {
            number_arrays.push(
                line.split(',')
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        }
    }

    let mut middle_number_sum = 0;
    for number_array in number_arrays {
        if !do_numbers_match_rules(&numbers_after_this_number, &number_array) {
            middle_number_sum +=
                order_numbers_according_to_rules(&numbers_after_this_number, &number_array);
        }
    }

    println!("Sum: {middle_number_sum:?}");

    Ok(())
}
