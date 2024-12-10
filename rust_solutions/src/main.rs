use std::io;

use common::read_input;

mod common;
mod day10;

fn main() -> io::Result<()> {
    let lines = read_input("inputs/day10.txt")?;
    let result = day10::_day10_part1(&lines)?;

    println!("The result is {result}");
    Ok(())
}
