use std::{collections::VecDeque, io};

use glam::IVec2;

use crate::common::Mat;

fn unpack_movement(movement_char: char) -> IVec2 {
    match movement_char {
        '^' => IVec2 { x: -1, y: 0 },
        'v' => IVec2 { x: 1, y: 0 },
        '<' => IVec2 { x: 0, y: -1 },
        '>' => IVec2 { x: 0, y: 1 },
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapItem {
    RobotPosition,
    BoxLeft,
    BoxRight,
    MapEdge,
    EmptySpace,
}

impl MapItem {
    fn from_char(ch: char) -> [Self; 2] {
        match ch {
            '#' => [Self::MapEdge, Self::MapEdge],
            '.' => [Self::EmptySpace, Self::EmptySpace],
            'O' => [Self::BoxLeft, Self::BoxRight],
            '@' => [Self::RobotPosition, Self::EmptySpace],
            _ => unreachable!(),
        }
    }

    fn _to_char(&self) -> char {
        match &self {
            MapItem::RobotPosition => '@',
            MapItem::BoxLeft => '[',
            MapItem::BoxRight => ']',
            MapItem::MapEdge => '#',
            MapItem::EmptySpace => '.',
        }
    }

    fn is_box(&self) -> bool {
        match self {
            MapItem::BoxLeft | MapItem::BoxRight => true,
            _ => false,
        }
    }
}

fn _print_map(map: &Mat<MapItem>) {
    for line in &map.data {
        let string_row = line.iter().map(|item| item._to_char()).collect::<String>();
        println!("{string_row:?}");
    }
}

fn try_move_box(map: &mut Mat<MapItem>, robot_position: &IVec2, movement: &IVec2) -> bool {
    let box_position = robot_position + movement;
    let box_type = map.at(&box_position).unwrap();
    let mut boxes_to_process = VecDeque::new();

    // Add the left side of the box
    boxes_to_process.push_back(if *box_type == MapItem::BoxLeft {
        box_position
    } else {
        box_position - IVec2::Y
    });

    let mut can_move_boxes = true;
    let mut all_boxes_to_move = vec![boxes_to_process[0]];

    while let Some(box_position) = boxes_to_process.pop_front() {
        if !can_move_boxes {
            break;
        }

        let new_box = box_position + movement;
        let boxes_to_check = if movement.x == 0 {
            // If the movement is on the y axis, then only 1 box might be moved
            // In this case we have to add the movement again if we are moving to the right, because
            // the box has a width of 2 and new_box is only 1 unit away,
            // so it will be the other part of the box
            if movement.y == -1 {
                vec![new_box]
            } else {
                vec![new_box + movement]
            }
        } else {
            // If the movement is on the x axis, then 2 boxes can also be influenced
            vec![new_box, new_box + IVec2::Y]
        };
        for box_coordinate in &boxes_to_check {
            let item_type = map.at(&box_coordinate).unwrap();
            if item_type.is_box() {
                if *item_type == MapItem::BoxLeft {
                    if !boxes_to_process.contains(&box_coordinate) {
                        boxes_to_process.push_back(box_coordinate.clone());
                        all_boxes_to_move.push(box_coordinate.clone());
                    }
                    break; // Don't add this box when processing the right side as well
                } else {
                    if !boxes_to_process.contains(&box_coordinate) {
                        boxes_to_process.push_back(box_coordinate - IVec2::Y);
                        all_boxes_to_move.push(box_coordinate - IVec2::Y);
                    }
                }
            } else if *item_type == MapItem::MapEdge {
                can_move_boxes = false;
                break;
            }
        }
    }

    if can_move_boxes {
        // Process the boxes in reverse order, so we can move the closer boxes to the place of the farther ones
        while let Some(box_to_move) = all_boxes_to_move.pop() {
            if movement.x == 0 {
                if movement.y == 1 {
                    // Moving to the right
                    map.set_at(&(box_to_move), &MapItem::EmptySpace);
                    map.set_at(&(box_to_move + movement), &MapItem::BoxLeft);
                    map.set_at(&(box_to_move + 2 * movement), &MapItem::BoxRight);
                } else {
                    // Moving to the left
                    map.set_at(&(box_to_move - movement), &MapItem::EmptySpace);
                    map.set_at(&(box_to_move), &MapItem::BoxRight);
                    map.set_at(&(box_to_move + movement), &MapItem::BoxLeft);
                }
            } else {
                map.set_at(&(box_to_move + movement), &MapItem::BoxLeft);
                map.set_at(&(box_to_move + movement + IVec2::Y), &MapItem::BoxRight);
                map.set_at(&(box_to_move), &MapItem::EmptySpace);
                map.set_at(&(box_to_move + IVec2::Y), &MapItem::EmptySpace);
            }
        }
    }

    can_move_boxes
}

fn perform_movements(map: &mut Mat<MapItem>, robot_position: &IVec2, movements: &Vec<IVec2>) {
    let mut position = *robot_position;
    for movement in movements {
        // let mut line = String::new();
        // let _ = std::io::stdin().read_line(&mut line).unwrap();
        let possible_new_pos = position + *movement;
        let cell_to_go_to = *map.at(&possible_new_pos).unwrap();

        if cell_to_go_to == MapItem::MapEdge {
            continue;
        } else if cell_to_go_to == MapItem::EmptySpace {
            map.set_at(&position, &MapItem::EmptySpace);
            map.set_at(&possible_new_pos, &MapItem::RobotPosition);
            position = possible_new_pos;
        } else if cell_to_go_to == MapItem::BoxLeft || cell_to_go_to == MapItem::BoxRight {
            if try_move_box(map, &position, movement) {
                map.set_at(&position, &MapItem::EmptySpace);
                map.set_at(&possible_new_pos, &MapItem::RobotPosition);
                position = possible_new_pos;
            }
        }

        // print!("\x1B[2J\x1B[1;1H");
        // println!("{movement:?}");
        // print_map(map);
    }
}

fn sum_box_coordinates(map: &Mat<MapItem>) -> usize {
    let mut sum = 0;
    for (i, row) in map.data.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == MapItem::BoxLeft {
                sum += 100 * i + j;
            }
        }
    }

    sum
}

pub fn solve(input_lines: &[String]) -> io::Result<usize> {
    let mut map = Mat::new();
    let mut movements = Vec::new();

    let mut parsing_map = true;

    for line in input_lines {
        if parsing_map {
            if line.is_empty() {
                parsing_map = false;
                continue;
            }

            let items = line
                .as_bytes()
                .iter()
                .map(|value| MapItem::from_char(*value as char))
                .collect::<Vec<_>>();
            let mut final_items = Vec::new();
            for item in items {
                final_items.push(item[0]);
                final_items.push(item[1]);
            }

            map.add_row(final_items);
        } else {
            movements.extend_from_slice(
                &line
                    .as_bytes()
                    .iter()
                    .map(|value| unpack_movement(*value as char))
                    .collect::<Vec<_>>(),
            );
        }
    }

    let robot_starting_position = map.find(&MapItem::RobotPosition).unwrap();
    perform_movements(&mut map, &robot_starting_position, &movements);

    println!("{map:?}");
    // println!("{movements:?}");

    let result = sum_box_coordinates(&map);

    Ok(result)
}
