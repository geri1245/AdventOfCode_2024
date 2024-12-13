use std::io;

use common::read_input;

mod common;
mod day13;

fn main() -> io::Result<()> {
    let lines = read_input("inputs/day13.txt")?;
    let result = day13::solve(&lines)?;

    println!("The result is {result}");
    Ok(())
}
