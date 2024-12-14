use std::io;

use common::read_input;

mod common;
mod day14;

fn main() -> io::Result<()> {
    let lines = read_input("inputs/day14.txt")?;
    let result = day14::solve(&lines)?;

    println!("The result is {result}");
    Ok(())
}
