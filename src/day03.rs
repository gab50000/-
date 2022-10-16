use std::io;

fn count_trees(lines: &str, right: usize, down: usize) -> usize {
    lines
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.as_bytes()[(i * right) % line.len()]))
        .map(|(i, x)| (i, x as char))
        .map(|(i, x)| i % down == 0 && x == '#')
        .map(|x| x as usize)
        .sum()
}

fn count_trees2(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let (mut i, mut j) = (0, 0);
    let mut count = 0;
    while i < map.len() {
        if map[i][j % map[i].len()] == '#' {
            count += 1;
        }
        i += down;
        j += right;
    }
    count
}

fn read_map(lines: &str) -> Vec<Vec<char>> {
    lines.lines().map(|line| line.chars().collect()).collect()
}

pub fn solve_a() -> io::Result<()> {
    let lines = std::fs::read_to_string("data/03.txt")?;
    let count = count_trees(&lines, 3, 1);
    println!("Count: {}", count);
    let map = read_map(&lines);
    println!("Count: {}", count_trees2(&map, 3, 1));
    Ok(())
}

pub fn solve_b() -> io::Result<()> {
    let lines = std::fs::read_to_string("data/03.txt")?;
    let map = read_map(&lines);

    let solution: Vec<usize> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| count_trees2(&map, right, down))
        .collect();

    println!("Solutions: {:?}", solution);
    println!("Solutions: {:?}", solution.into_iter().product::<usize>());
    // println!("Total: {:?}", map);

    Ok(())
}
