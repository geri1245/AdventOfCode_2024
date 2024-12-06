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
    Unvisited,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '.' => CellType::Unvisited,
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

fn count_cells_traversed(
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

    let cells_visited = count_cells_traversed(&mut map, guard_position, IVec2::new(-1, 0));

    println!("Number of cells visited: {cells_visited:?}");
    Ok(())
}
