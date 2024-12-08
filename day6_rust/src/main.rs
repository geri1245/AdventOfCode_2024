use std::{
    collections::HashSet,
    fs::File,
    io::{self, Read},
    path::Path,
};

use glam::IVec2;

#[derive(Debug)]
enum CellType {
    Obstacle,
    CurrentPosition,
    FreeSpace,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '.' => CellType::FreeSpace,
            '#' => CellType::Obstacle,
            '^' => CellType::CurrentPosition,
            _ => panic!("This character should never be converted to CellType"),
        }
    }
}

fn rotate_90_clockwise(vec: IVec2) -> IVec2 {
    match vec {
        IVec2 { x: 1, y: 0 } => IVec2::new(0, -1),
        IVec2 { x: -1, y: 0 } => IVec2::new(0, 1),
        IVec2 { x: 0, y: 1 } => IVec2::new(1, 0),
        IVec2 { x: 0, y: -1 } => IVec2::new(-1, 0),
        _ => panic!("This is not a valid direction vector in the current context!"),
    }
}

fn at<'a>(map: &'a mut Vec<Vec<CellType>>, index: &IVec2) -> Option<&'a CellType> {
    if index.min_element() >= 0
        && (index.x as usize) < map.len()
        && (index.y as usize) < map[index.x as usize].len()
    {
        Some(&map[index.x as usize][index.y as usize])
    } else {
        None
    }
}

// Part 1
fn _count_cells_traversed(
    map: &mut Vec<Vec<CellType>>,
    current_position: IVec2,
    current_direction: IVec2,
) -> usize {
    let mut current_position = current_position;
    let mut current_direction = current_direction;

    let mut visited_cells = HashSet::new();
    visited_cells.insert(current_position);

    loop {
        let possible_next_position = current_position + current_direction;

        if let Some(cell) = at(map, &possible_next_position) {
            if matches!(cell, CellType::Obstacle) {
                current_direction = rotate_90_clockwise(current_direction);
            } else {
                current_position = possible_next_position;
                visited_cells.insert(current_position);
            }
        } else {
            break;
        }
    }

    visited_cells.len()
}

fn will_gurad_get_stuck_in_a_loop(
    map: &mut Vec<Vec<CellType>>,
    current_position: IVec2,
    current_direction: IVec2,
) -> bool {
    let mut current_position = current_position;
    let mut current_direction = current_direction;

    let mut visited_cells = HashSet::new();
    visited_cells.insert((current_position, current_direction));

    let mut was_stuck_in_loop = false;
    loop {
        let possible_next_position = current_position + current_direction;

        if let Some(cell) = at(map, &possible_next_position) {
            if matches!(cell, CellType::Obstacle) {
                current_direction = rotate_90_clockwise(current_direction);
            } else {
                current_position = possible_next_position;
                if !visited_cells.insert((current_position, current_direction)) {
                    // If the guard has already been to this position with this direction, that means it will be a loop
                    was_stuck_in_loop = true;
                    break;
                }
            }
        } else {
            break;
        }
    }

    was_stuck_in_loop
}

fn count_loop_obstacle_placements(
    map: &mut Vec<Vec<CellType>>,
    current_position: IVec2,
    current_direction: IVec2,
) -> u32 {
    let mut obstacle_positions_causing_loop = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if matches!(map[i][j], CellType::FreeSpace) {
                map[i][j] = CellType::Obstacle;
                if will_gurad_get_stuck_in_a_loop(map, current_position, current_direction) {
                    obstacle_positions_causing_loop += 1;
                }
                map[i][j] = CellType::FreeSpace;
            }
        }
    }
    obstacle_positions_causing_loop
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("input.txt");

    let mut input_file = File::open(path)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    let mut guard_position = IVec2::new(0, 0);
    let mut map = Vec::new();
    for row in content.lines() {
        map.push(
            row.as_bytes()
                .iter()
                .map(|byte| CellType::from(*byte as char))
                .collect::<Vec<_>>(),
        );

        if let Some(index) = map
            .last()
            .unwrap()
            .iter()
            .position(|item| matches!(item, CellType::CurrentPosition))
        {
            guard_position = IVec2::new(map.len() as i32 - 1, index as i32)
        }
    }

    let num_of_obstacles_causing_loops =
        count_loop_obstacle_placements(&mut map, guard_position, IVec2::new(-1, 0));

    println!("Number of cells visited: {num_of_obstacles_causing_loops:?}");
    Ok(())
}
