use std::{
    collections::{HashMap, HashSet},
    io,
};

use glam::IVec2;

use crate::common::Mat;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct PathState {
    direction: IVec2,
    position: IVec2,
}

fn can_traverse(map: &Mat<char>, coords: &IVec2) -> bool {
    *map.at(coords).unwrap() != '#'
}

fn is_goal_cell(map: &Mat<char>, coords: &IVec2) -> bool {
    *map.at(coords).unwrap() == 'E'
}

fn get_vectors_with_cost(vec: IVec2) -> [(IVec2, usize); 3] {
    if vec.x == 0 {
        [
            (vec, 1),
            (IVec2::new(1, 0), 1001),
            (IVec2::new(-1, 0), 1001),
        ]
    } else {
        [
            (vec, 1),
            (IVec2::new(0, 1), 1001),
            (IVec2::new(0, -1), 1001),
        ]
    }
}

fn find_optimal_seats(mat: &Mat<char>, reindeer_starting_pos: &IVec2) -> usize {
    let state = PathState {
        direction: IVec2::Y,
        position: *reindeer_starting_pos,
    };

    let mut paths = Vec::new();

    paths.push((state.clone(), 0));

    let mut items_seen = HashMap::new();

    while let Some((current_state, current_cost)) = paths.pop() {
        for (new_dir, current_move_cost) in get_vectors_with_cost(current_state.direction) {
            let new_pos = current_state.position + new_dir;
            if can_traverse(mat, &new_pos) {
                let new_state = PathState {
                    direction: new_dir,
                    position: current_state.position + new_dir,
                };
                if let Some((existing_score, paths_leading_here)) = items_seen.get_mut(&new_state) {
                    let new_score = current_cost + current_move_cost;
                    if new_score < *existing_score {
                        *existing_score = new_score;
                        *paths_leading_here = vec![current_state];
                        if !is_goal_cell(mat, &new_state.position) {
                            paths.push((new_state, new_score));
                        }
                    } else if new_score == *existing_score {
                        paths_leading_here.push(current_state);
                    }
                } else {
                    let new_cost = current_cost + current_move_cost;
                    if !is_goal_cell(mat, &new_state.position) {
                        paths.push((new_state, new_cost));
                    }
                    items_seen.insert(new_state, (new_cost, vec![current_state]));
                }
            }
        }
    }

    let goal_cell = mat.find(&'E').unwrap();

    let mut paths_leading_to_goal = Vec::new();
    let mut min_value = usize::MAX;

    for (state, (cost, paths_leading_here)) in &items_seen {
        if state.position == goal_cell && *cost < min_value {
            min_value = *cost;
            paths_leading_to_goal = paths_leading_here.clone();
        } else if state.position == goal_cell && *cost == min_value {
            paths_leading_to_goal.extend_from_slice(&paths_leading_here);
        }
    }

    let mut cells_around_optimal_path = HashSet::new();
    cells_around_optimal_path.insert(goal_cell);

    while let Some(state) = paths_leading_to_goal.pop() {
        if state.position == *reindeer_starting_pos {
            continue;
        }
        cells_around_optimal_path.insert(state.position);
        paths_leading_to_goal.extend(&items_seen.get(&state).unwrap().1);
    }

    cells_around_optimal_path.len() + 1
}

pub fn solve(input_lines: &[String]) -> io::Result<usize> {
    let mut mat = Mat::new();

    for line in input_lines {
        let items = line
            .as_bytes()
            .iter()
            .map(|value| *value as char)
            .collect::<Vec<_>>();

        mat.add_row(items);
    }

    let reindeer_start_pos = mat.find(&'S').unwrap();
    mat.set_at(&reindeer_start_pos, &'.');

    let result = find_optimal_seats(&mut mat, &reindeer_start_pos);

    Ok(result)
}
