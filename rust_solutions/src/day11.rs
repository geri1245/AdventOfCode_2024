use std::{collections::HashMap, io};

fn insert_with_default(number: u64, count: u64, map: &mut HashMap<u64, u64>) {
    if let Some(existing_count) = map.get_mut(&number) {
        *existing_count += count;
    } else {
        map.insert(number, count);
    }
}

fn perform_rule(number: u64, count: u64, destination: &mut HashMap<u64, u64>) {
    if number == 0 {
        insert_with_default(1, count, destination);
    } else if number.to_string().len() % 2 == 0 {
        let number_as_str = number.to_string();
        let (num1, num2) = number_as_str.split_at(number_as_str.len() / 2);
        insert_with_default(num1.parse::<u64>().unwrap(), count, destination);
        insert_with_default(num2.parse::<u64>().unwrap(), count, destination);
    } else {
        insert_with_default(number * 2024, count, destination);
    }
}

pub fn _day11(input_lines: &[String]) -> io::Result<usize> {
    let mut map1 = input_lines[0]
        .split(' ')
        .map(|num_str| (num_str.parse::<u64>().unwrap(), 1u64))
        .collect::<HashMap<_, _>>();

    let mut map2 = HashMap::new();

    let mut is_vec1_destination = false;
    for _ in 0..75 {
        let (source, destination) = if is_vec1_destination {
            (map2.drain(), &mut map1)
        } else {
            (map1.drain(), &mut map2)
        };

        for (num, count) in source {
            perform_rule(num, count, destination);
        }

        is_vec1_destination = !is_vec1_destination;
    }

    let result_vec = if is_vec1_destination { &map2 } else { &map1 };
    let result = result_vec.values().map(|num| *num).sum::<u64>();

    Ok(result as usize)
}
