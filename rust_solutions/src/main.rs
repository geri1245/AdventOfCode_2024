use std::io;

use common::read_input;

mod common;
mod day17;

fn main() -> io::Result<()> {
    let lines = read_input("inputs/day17.txt")?;
    let result = day17::solve(&lines)?;

    println!("The result is {result}");
    Ok(())
}
