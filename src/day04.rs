use std::{io, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::char,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn parse_numbers<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(digit1, T::from_str)(input)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BirthYear(u32);

impl BirthYear {
    fn parse(input: &str) -> IResult<&str, Self> {
        let prefix = tuple((tag("byr"), char(':')));
        let parser = preceded(prefix, parse_numbers::<u32>);
        let mut p = map(parser, Self);
        p(input)
    }
}

#[test]
fn test_birth_year() {
    assert_eq!(BirthYear::parse("byr:123"), Ok(("", BirthYear(123))));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IssueYear(u32);
#[derive(Debug, Clone, Copy, PartialEq)]
struct ExpirationYear(u32);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Height {
    Cm(u32),
    In(u32),
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct HairColor<'a>(&'a str);
#[derive(Debug, Clone, Copy, PartialEq)]
struct EyeColor<'a>(&'a str);
#[derive(Debug, Clone, Copy, PartialEq)]
struct PassportID(u32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct CountryID(u32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Passport<'a> {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpirationYear,
    hgt: Height,
    hcl: HairColor<'a>,
    ecl: EyeColor<'a>,
    pid: PassportID,
    cid: CountryID,
}

pub fn solve_a() -> io::Result<()> {
    println!("{:?}", CountryID(123));
    Ok(())
}
