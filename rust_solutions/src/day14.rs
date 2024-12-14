use std::io::{self, stdout, Write};

use glam::IVec2;

const MAP_SIZE: IVec2 = IVec2 { x: 101, y: 103 };
const DURATION: i32 = 100;

fn parse_tuple(string: &str) -> IVec2 {
    let comma_index = string.find(',').unwrap();
    let left = string[2..comma_index].parse::<i32>().unwrap();
    let right = string[(comma_index + 1)..].parse::<i32>().unwrap();
    IVec2::new(left, right)
}

fn parse_line(line: &String) -> (IVec2, IVec2) {
    let mut parts = line.split(' ');
    let position_part = parts.next().unwrap();
    let direction_part = parts.next().unwrap();

    let position = parse_tuple(position_part);
    let direction = parse_tuple(direction_part);

    (position, direction)
}

fn get_quadrant(pos: &IVec2) -> Option<usize> {
    let half_coords = MAP_SIZE / 2;
    if pos.x < half_coords.x {
        if pos.y < half_coords.y {
            Some(0)
        } else if pos.y > half_coords.y {
            Some(2)
        } else {
            None
        }
    } else if pos.x > half_coords.x {
        if pos.y < half_coords.y {
            Some(1)
        } else if pos.y > half_coords.y {
            Some(3)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn solve(input_lines: &[String]) -> io::Result<usize> {
    let mut results = [0; 4];
    let mut robots = Vec::new();
    for line in input_lines {
        let (pos, dir) = parse_line(&line);
        robots.push((pos, dir));

        let final_pos = (pos + DURATION * dir).rem_euclid(MAP_SIZE);
        if let Some(quadrant) = get_quadrant(&final_pos) {
            results[quadrant] += 1;
        }
    }

    let mut ind = 0;
    let mut forward = true;
    loop {
        // let result = std::io::stdin().read_line(&mut line);
        let mut positions = vec![vec![' '; 103]; 101];
        for (pos, dir) in &mut robots {
            if forward {
                *pos = (*pos + *dir).rem_euclid(MAP_SIZE);
            } else {
                *pos = (*pos - *dir).rem_euclid(MAP_SIZE);
            }
            positions[pos.x as usize][pos.y as usize] = 'x';
        }

        // clear terminal and jump to position 1,1
        print!("\x1B[2J\x1B[1;1H");
        println!("iter {ind}");
        for line in positions {
            let string = line.iter().collect::<String>();
            println!("{string}");
        }
        let _ = stdout().flush();
        let mut line = String::new();
        let result = std::io::stdin().read_line(&mut line).unwrap();
        // \r\n will be 2, so just pressing enter will go forward
        if result == 2 {
            forward = true;
            ind += 1;
        } else {
            forward = false;
            ind -= 1;
        }
    }

    Ok(results[0] * results[1] * results[2] * results[3])
}
