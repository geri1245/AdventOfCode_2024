use std::io;

use glam::IVec2;

const POSSIBLE_STEPS: [IVec2; 4] = [
    IVec2 { x: 1, y: 0 },
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 0, y: -1 },
];

fn is_coord_valid(coord: &IVec2, mat: &Vec<Vec<i32>>) -> bool {
    coord.min_element() >= 0 && (coord.x as usize) < mat.len() && (coord.y as usize) < mat[0].len()
}

fn at(coord: &IVec2, mat: &Vec<Vec<i32>>) -> i32 {
    mat[coord.x as usize][coord.y as usize]
}

fn calculate_trail_score(start_pos: &IVec2, mat: &Vec<Vec<i32>>) -> usize {
    let mut paths = vec![*start_pos];
    let mut finished_trails = Vec::new();

    while !paths.is_empty() {
        let mut new_paths = vec![];
        for current_pos in &paths {
            for step in POSSIBLE_STEPS {
                let next_step = *current_pos + step;
                if is_coord_valid(&next_step, mat) {
                    if at(current_pos, mat) + 1 == at(&next_step, mat) {
                        if at(&next_step, mat) == 9 {
                            finished_trails.push(next_step);
                            println!("finished at {next_step:?}");
                        } else {
                            new_paths.push(next_step);
                        }
                    }
                }
            }
        }

        paths = new_paths;
    }

    finished_trails.len()
}

pub fn _day10_part1(input_lines: &[String]) -> io::Result<usize> {
    let mut mat = Vec::new();
    for line in input_lines {
        mat.push(
            line.as_bytes()
                .iter()
                .map(|byte| {
                    if (*byte as char) == '.' {
                        -1
                    } else {
                        (*byte as char).to_string().parse::<i32>().unwrap()
                    }
                })
                .collect::<Vec<i32>>(),
        );
    }

    let mut start_positions = Vec::new();

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == 0 {
                start_positions.push(IVec2::new(i as i32, j as i32));
            }
        }
    }

    let mut result = 0;
    println!("{start_positions:?}");
    for start_pos in start_positions {
        result += calculate_trail_score(&start_pos, &mat);
    }

    Ok(result)
}
