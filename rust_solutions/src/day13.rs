use std::io;

use glam::{DMat2, DVec2};

const FIRST_TEXT: &str = "Button A: X+";
const PRIZE_TEXT: &str = "Prize: X=";

fn get_equation_line(line: &String) -> (u64, u64) {
    let comma_index = line.find(',').unwrap();
    let x = line[FIRST_TEXT.len()..comma_index].parse::<u64>().unwrap();
    let y = line[(comma_index + 3)..].parse::<u64>().unwrap();
    (x, y)
}

fn get_result_line(line: &String) -> (u64, u64) {
    let comma_index = line.find(',').unwrap();
    let x = line[PRIZE_TEXT.len()..comma_index].parse::<u64>().unwrap();
    let y = line[(comma_index + 4)..].parse::<u64>().unwrap();
    (x, y)
}

pub fn solve(input_lines: &[String]) -> io::Result<usize> {
    let equations = input_lines
        .chunks(4)
        .map(|chunk| {
            (
                get_equation_line(&chunk[0]),
                get_equation_line(&chunk[1]),
                get_result_line(&chunk[2]),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for ((x1, y1), (x2, y2), (x, y)) in equations {
        let x = x + 10000000000000;
        let y = y + 10000000000000;
        // n x x1 + m x x2 == a -> we are looking for n and m

        let coeffs_mat = DMat2::from_cols(
            DVec2::new(x1 as f64, y1 as f64),
            DVec2::new(x2 as f64, y2 as f64),
        );
        let result_vec = DVec2::new(x as f64, y as f64);

        let solution_vec = coeffs_mat.inverse() * result_vec;

        if solution_vec.min_element() < 0.0 {
            continue;
        }
        let rounded_solution_vec = solution_vec.round();
        if !solution_vec.abs_diff_eq(rounded_solution_vec, 0.001) {
            continue;
        }

        sum += (rounded_solution_vec.x as usize) * 3 + (rounded_solution_vec.y as usize);
    }

    Ok(sum)
}
