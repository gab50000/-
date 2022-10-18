use std::io;

pub fn solve_a() -> io::Result<()> {
    let numbers = read_numbers("./data/01a.txt")?;
    if let Some(result) = find_2020_tuple(numbers) {
        println!("{}", result.0 * result.1);
    }
    Ok(())
}

pub fn solve_b() -> io::Result<()> {
    let numbers = read_numbers("./data/01a.txt")?;
    if let Some(result) = find_2020_triple(numbers) {
        println!("{:?}", result);
        println!("{}", result.0 * result.1 * result.2);
    }
    Ok(())
}
fn read_numbers(filename: &str) -> io::Result<Vec<i32>> {
    let lines = std::fs::read_to_string(filename)?;
    let result = lines
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect();
    Ok(result)
}

fn find_2020_tuple(numbers: Vec<i32>) -> Option<(i32, i32)> {
    for i in 0..numbers.len() {
        for j in 0..i {
            if numbers[i] + numbers[j] == 2020 {
                return Some((numbers[i], numbers[j]));
            }
        }
    }
    None
}

fn find_2020_triple(numbers: Vec<i32>) -> Option<(i32, i32, i32)> {
    for i in 0..numbers.len() {
        for j in 0..i {
            for k in 0..j {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return Some((numbers[i], numbers[j], numbers[k]));
                }
            }
        }
    }
    None
}
