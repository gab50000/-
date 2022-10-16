use std::io;

pub fn solve_a() -> io::Result<()> {
    let lines = std::fs::read_to_string("data/03.txt")?;
    let count = lines
        .lines()
        .enumerate()
        .map(|(i, line)| line.as_bytes()[(i * 3) % line.len()])
        .map(|x| x as char)
        .filter(|&x| x == '#')
        .count();

    println!("Count: {}", count);
    Ok(())
}
