use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};
use std::{io, str::FromStr};

#[derive(Debug)]
struct Policy {
    char: char,
    min_occurrence: i32,
    max_occurrence: i32,
}

impl Policy {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let range_parser = separated_pair(parse_numbers, char('-'), parse_numbers);
        let parser = separated_pair(range_parser, char(' '), anychar);
        let mut p = map(parser, |((min, max), char)| Policy {
            min_occurrence: min,
            max_occurrence: max,
            char,
        });
        p(input)
    }
}

fn parse_line(input: &str) -> IResult<&str, (Policy, &str)> {
    let mut parser = separated_pair(Policy::parse, tag(": "), alpha1);
    parser(input)
}

fn parse_numbers(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}

pub fn solve_a() -> io::Result<()> {
    let lines = std::fs::read_to_string("data/02.txt")?;
    let result = lines.lines().map(parse_line).map(Result::unwrap);
    for (_, (pol, str)) in result {
        println!("{:?} -> {}", pol, str);
    }
    Ok(())
}
