use std::io;

pub fn solve_a() {
    if let Ok(numbers) = read_numbers("./data/01a.txt") {
        for num in numbers {
            println!("{}", num);
        }
    }
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
