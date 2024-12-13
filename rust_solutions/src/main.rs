use std::io;

use common::read_input;

mod common;
mod day15;

fn main() -> io::Result<()> {
    let lines = read_input("inputs/day15.txt")?;
    let result = day15::solve(&lines)?;

    println!("The result is {result}");
    Ok(())
}
