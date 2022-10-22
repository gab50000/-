use std::{io, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::char,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    IResult,
};

macro_rules! make_struct {
    ($id:ident, &str, $tag:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct $id<'a>(&'a str);
        impl<'a> $id<'a> {
            fn parse(input: &'a str) -> IResult<&'a str, Self> {
                let prefix = tuple((tag($tag), char(':')));
                let parser = preceded(prefix, alpha1);
                let mut p = map(parser, Self);
                p(input)
            }
        }
    };
    ($id:ident,$type:ty, $tag:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct $id($type);
        impl $id {
            fn parse(input: &str) -> IResult<&str, Self> {
                let prefix = tuple((tag($tag), char(':')));
                let parse_numbers = map_res(digit1, <$type>::from_str);
                let parser = preceded(prefix, parse_numbers);
                let mut p = map(parser, Self);
                p(input)
            }
        }
    };
}

make_struct!(BirthYear, u32, "byr");
make_struct!(IssueYear, u32, "iyr");
make_struct!(ExpirationYear, u32, "eyr");
make_struct!(Height, u32, "hgt");
make_struct!(HairColor, &str, "hcl");
make_struct!(EyeColor, &str, "ecl");
make_struct!(PassportID, u32, "pid");
make_struct!(CountryID, u32, "cid");

#[test]
fn test_birth_year() {
    assert_eq!(BirthYear::parse("byr:123"), Ok(("", BirthYear(123))));
}

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
