use std::{
    collections::{HashSet, VecDeque},
    io,
};

use glam::IVec2;

const POSSIBLE_STEPS: [IVec2; 4] = [
    IVec2 { x: 1, y: 0 },
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 0, y: -1 },
];

struct Node {
    value: u8,
    visited: bool,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Fence {
    coords: IVec2,
    is_horizontal: bool,
    is_on_positive_side: bool,
}

impl Fence {
    fn new(tile_coords: IVec2, step_direction: IVec2) -> Self {
        let (direction, is_on_positive_side) = if step_direction.min_element() < 0 {
            (IVec2::ZERO, false)
        } else {
            (step_direction, true)
        };
        let fence_coords = tile_coords + direction;
        let is_horizontal = step_direction == IVec2::X || step_direction == IVec2::NEG_X;
        Self {
            coords: fence_coords,
            is_horizontal,
            is_on_positive_side,
        }
    }

    fn neighbors(&self) -> [Fence; 2] {
        let directions = if self.is_horizontal {
            [IVec2::Y, IVec2::NEG_Y]
        } else {
            [IVec2::X, IVec2::NEG_X]
        };
        directions.map(|dir| Fence {
            coords: self.coords + dir,
            is_horizontal: self.is_horizontal,
            is_on_positive_side: self.is_on_positive_side,
        })
    }
}

fn is_coord_valid(coord: &IVec2, mat: &Vec<Vec<Node>>) -> bool {
    coord.min_element() >= 0 && (coord.x as usize) < mat.len() && (coord.y as usize) < mat[0].len()
}

fn at<'a>(coord: &IVec2, mat: &'a mut Vec<Vec<Node>>) -> &'a mut Node {
    &mut mat[coord.x as usize][coord.y as usize]
}

fn find_islands(mat: &mut Vec<Vec<Node>>) -> usize {
    let mut all_tiles_queue = VecDeque::new();
    all_tiles_queue.push_back(IVec2::new(0, 0));
    let mut value_sum = 0;
    // Process each tile
    while let Some(coord) = all_tiles_queue.pop_front() {
        if at(&coord, mat).visited {
            continue;
        }

        let mut current_patch_queue = VecDeque::new();
        current_patch_queue.push_back(coord);
        let mut tile_count = 0;
        let mut fences = HashSet::new();

        // Process each tile in the current patch and calculate its properties
        while let Some(current_patch_coord) = current_patch_queue.pop_front() {
            if at(&current_patch_coord, mat).visited {
                continue;
            }

            at(&current_patch_coord, mat).visited = true;
            tile_count += 1;

            for step in POSSIBLE_STEPS {
                let neighbor_coords = current_patch_coord + step;
                if !is_coord_valid(&neighbor_coords, mat) {
                    fences.insert(Fence::new(current_patch_coord, step));
                    continue;
                }

                let has_neighbor_been_visited = at(&neighbor_coords, mat).visited;
                let does_neighbor_have_same_value =
                    at(&neighbor_coords, mat).value == at(&current_patch_coord, mat).value;
                if has_neighbor_been_visited {
                    if !does_neighbor_have_same_value {
                        fences.insert(Fence::new(current_patch_coord, step));
                    }
                } else {
                    if does_neighbor_have_same_value {
                        current_patch_queue.push_back(neighbor_coords);
                    } else {
                        all_tiles_queue.push_back(neighbor_coords);
                        fences.insert(Fence::new(current_patch_coord, step));
                    }
                }
            }
        }

        let mut fence_sum = 0;
        let mut fences_visited = HashSet::new();

        for fence in &fences {
            if fences_visited.contains(fence) {
                continue;
            }

            fence_sum += 1;

            let mut fences_to_process = VecDeque::new();
            fences_to_process.push_back(*fence);

            while let Some(fence_in_line) = fences_to_process.pop_front() {
                if fences_visited.contains(&fence_in_line) {
                    continue;
                }
                fences_visited.insert(fence_in_line);
                for neighbor_fence in fence_in_line.neighbors() {
                    if fences.contains(&neighbor_fence) {
                        if !fences_visited.contains(&neighbor_fence) {
                            fences_to_process.push_back(neighbor_fence);
                        }
                    }
                }
            }
        }

        println!("Tile count: {tile_count}");
        println!("Fence sum: {fence_sum}");
        value_sum += tile_count * fence_sum;
    }

    value_sum
}

pub fn _day12(input_lines: &[String]) -> io::Result<usize> {
    let mut mat = Vec::new();
    for line in input_lines {
        mat.push(
            line.as_bytes()
                .iter()
                .map(|value| Node {
                    value: *value,
                    visited: false,
                })
                .collect::<Vec<_>>(),
        );
    }

    let result = find_islands(&mut mat);
    Ok(result)
}
