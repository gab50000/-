use std::{io, str::FromStr};

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::char,
    character::complete::{alphanumeric1, digit1, line_ending, space0},
    combinator::{map, map_res},
    sequence::{preceded, terminated, tuple},
    IResult,
};

// fn le_parser(input: &str) -> IResult<&str, &str> {
//     line_ending(input)
// }

macro_rules! make_struct {
    ($id:ident, &str, $tag:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct $id<'a>(&'a str);
        impl<'a> $id<'a> {
            fn parse(input: &'a str) -> IResult<&'a str, Self> {
                let prefix = tuple((tag($tag), char(':')));
                let space_or_newline = alt((space0, line_ending));
                let parser = terminated(preceded(prefix, alphanumeric1), space_or_newline);
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
                let space_or_newline = alt((space0, line_ending));
                let parser = terminated(preceded(prefix, parse_numbers), space_or_newline);
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
    pub byr: Option<BirthYear>,
    pub iyr: Option<IssueYear>,
    pub eyr: Option<ExpirationYear>,
    pub hgt: Option<Height>,
    pub hcl: Option<HairColor<'a>>,
    pub ecl: Option<EyeColor<'a>>,
    pub pid: Option<PassportID>,
    pub cid: Option<CountryID>,
}

impl<'a> Passport<'a> {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn parse(input: &'a str) -> Self {
        let mut p = permutation((
            BirthYear::parse,
            IssueYear::parse,
            ExpirationYear::parse,
            Height::parse,
            HairColor::parse,
            EyeColor::parse,
            PassportID::parse,
            CountryID::parse,
        ));
        let result = p(input);

        let mut passport = Passport::new();
        if let Ok((_, values)) = result {
            passport.byr = Some(values.0);
            passport.iyr = Some(values.1);
            passport.eyr = Some(values.2);
            passport.hgt = Some(values.3);
            passport.hcl = Some(values.4);
            passport.ecl = Some(values.5);
            passport.pid = Some(values.6);
            passport.cid = Some(values.7);
        }
        passport
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

#[test]
fn test_passport() {
    let p = Passport::new();
    assert!(!p.is_valid());
}

pub fn solve_a() -> io::Result<()> {
    let p =
        Passport::parse("hgt:172 pid:170 hcl:17106b iyr:2012 ecl:gry cid:123 eyr:2020 byr:1990");
    println!("{:?}", p);
    // Passport::parse("hgt:172in pid:170cm hcl:17106b iyr:2012 ecl:gry cid:123 eyr:2020 byr:1990");
    Ok(())
}
