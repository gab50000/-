use std::io;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> io::Result<()> {
    day01::solve_a()?;
    day01::solve_b()?;
    day02::solve_a()?;
    day02::solve_b()?;
    day03::solve_a()?;
    day03::solve_b()?;
    day04::solve_a()?;
    Ok(())
}
