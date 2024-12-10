use std::io;

use crate::common::read_input;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Multiply,
    Concat,
    Add,
}

#[derive(Clone, Copy, Debug)]
enum MathItem {
    Number(u64),
    Operation(Operator),
}

impl MathItem {
    fn get_number(&self) -> u64 {
        match self {
            MathItem::Number(num) => *num,
            MathItem::Operation(_) => panic!(""),
        }
    }

    fn get_operation(&self) -> Operator {
        match self {
            MathItem::Number(_) => panic!(""),
            MathItem::Operation(op) => *op,
        }
    }
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    (lhs.to_string() + &rhs.to_string())
        .as_str()
        .parse::<u64>()
        .unwrap()
}

pub fn _day7_part2(input_file: &str) -> io::Result<()> {
    let mut result = 0;

    for line in read_input(input_file)? {
        let parts = line.split(':').collect::<Vec<_>>();
        let expected_result = parts[0].parse::<u64>().unwrap();
        let numbers = parts[1]
            .trim_start()
            .split(' ')
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        // Contains the full sum and the latest multiplication result
        let mut equations: Vec<Vec<MathItem>> = vec![vec![MathItem::Number(numbers[0])]];

        for number in numbers.iter().skip(1) {
            for i in 0..equations.len() {
                let mut add_copy = equations[i].clone();
                let mut concat_copy = equations[i].clone();

                equations[i].push(MathItem::Operation(Operator::Multiply));
                add_copy.push(MathItem::Operation(Operator::Add));
                concat_copy.push(MathItem::Operation(Operator::Concat));

                equations.push(add_copy);
                equations.push(concat_copy);
            }

            for eq in &mut equations {
                eq.push(MathItem::Number(*number));
            }
        }

        let mut actual_results = Vec::new();
        for equation in equations {
            let mut i = 1;
            let mut current_sum = equation[0].get_number();
            while i < equation.len() {
                match equation[i].get_operation() {
                    Operator::Multiply => current_sum = current_sum * equation[i + 1].get_number(),
                    Operator::Concat => {
                        current_sum = concat(current_sum, equation[i + 1].get_number())
                    }
                    Operator::Add => current_sum = current_sum + equation[i + 1].get_number(),
                }
                i += 2;
            }
            actual_results.push(current_sum);
        }

        let is_row_correct = actual_results
            .iter()
            .any(|actual_result| *actual_result == expected_result);

        if is_row_correct {
            result += expected_result;
        }
    }

    println!("The result is: {result}");

    Ok(())
}

pub fn _day7_part1(input_file: &str) -> io::Result<()> {
    let mut result = 0;

    for line in read_input(input_file)? {
        let parts = line.split(':').collect::<Vec<_>>();
        let expected_result = parts[0].parse::<u64>().unwrap();
        let numbers = parts[1]
            .trim_start()
            .split(' ')
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        // Contains the full sum and the latest multiplication result
        let mut sums = vec![0];

        for number in numbers {
            for i in 0..sums.len() {
                let sum = sums[i];
                sums[i] = sum + number;
                sums.push(sum * number);
            }
        }

        let is_row_correct = sums
            .into_iter()
            .any(|actual_result| actual_result == expected_result);

        if is_row_correct {
            result += expected_result;
        }
    }

    println!("The result is: {result}");

    Ok(())
}
