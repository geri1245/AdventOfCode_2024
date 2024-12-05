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
        if do_numbers_match_rules(&numbers_after_this_number, &number_array) {
            middle_number_sum += number_array[number_array.len() / 2];
        }
    }

    println!("Sum: {middle_number_sum:?}");

    Ok(())
}
