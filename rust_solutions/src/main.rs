use std::io;

use common::read_input;

mod common;
mod day16;

fn main() -> io::Result<()> {
    let lines = read_input("inputs/day16.txt")?;
    let result = day16::solve(&lines)?;

    println!("The result is {result}");
    Ok(())
}
