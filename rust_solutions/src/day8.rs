use std::{
    collections::{HashMap, HashSet},
    io,
};

use glam::IVec2;

use crate::common::read_input;

fn is_coord_inside_map(coord: &IVec2, bounds: &IVec2) -> bool {
    coord.min_element() >= 0 && coord.x < bounds.x && coord.y < bounds.y
}

fn get_num_of_antinodes(
    loc1: &IVec2,
    loc2: &IVec2,
    bounds: &IVec2,
    antinode_locations: &mut HashSet<IVec2>,
) {
    let difference = loc2 - loc1;
    let mut i = 0;
    loop {
        let possible_location = loc2 + i * difference;
        if is_coord_inside_map(&possible_location, bounds) {
            antinode_locations.insert(possible_location);
            i += 1;
        } else {
            break;
        }
    }

    i = 0;
    loop {
        let possible_location = loc1 + i * difference;
        if is_coord_inside_map(&possible_location, bounds) {
            antinode_locations.insert(possible_location);
            i -= 1;
        } else {
            break;
        }
    }
}

pub fn _day8_part1(input_file: &str) -> io::Result<u64> {
    let mut antenna_positions: HashMap<char, Vec<IVec2>> = HashMap::new();
    let input_lines = read_input(input_file)?;
    let dimensions = IVec2::new(
        input_lines.len() as i32,
        input_lines[0].as_bytes().len() as i32,
    );

    for (i_x, line) in input_lines.iter().enumerate() {
        for (i_y, byte) in line.as_bytes().iter().enumerate() {
            let char = *byte as char;

            if char != '.' {
                let coords = IVec2::new(i_x as i32, i_y as i32);
                if let Some(positions) = antenna_positions.get_mut(&char) {
                    positions.push(coords);
                } else {
                    antenna_positions.insert(char, vec![coords]);
                }
            }
        }
    }

    let mut antinode_locations = HashSet::new();
    for (_, antenna_locations) in antenna_positions {
        for i in 0..(antenna_locations.len() - 1) {
            for j in i + 1..antenna_locations.len() {
                get_num_of_antinodes(
                    &antenna_locations[i],
                    &antenna_locations[j],
                    &dimensions,
                    &mut antinode_locations,
                );
            }
        }
    }

    Ok(antinode_locations.len() as u64)
}
