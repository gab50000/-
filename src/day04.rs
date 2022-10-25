use std::{fmt::Error, io, str::FromStr};

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::char,
    character::complete::{alphanumeric1, digit1, line_ending, space0},
    combinator::{map, map_res},
    sequence::{preceded, terminated, tuple},
    IResult,
};

macro_rules! prefix {
    ($tag:literal) => {
        tuple((tag($tag), char(':')))
    };
}

macro_rules! parse_num {
    ($prefix:tt) => {
        fn parse(input: &str) -> IResult<&str, Self> {
            let mut prefix = prefix!($prefix);
            let (input, _) = prefix(input)?;
            let (input, val) = map_res(digit1, u32::from_str)(input)?;
            Ok((input, Self(val)))
        }
    };
}
macro_rules! parse_str {
    ($prefix:tt) => {
        fn parse(input: &'a str) -> IResult<&str, Self> {
            let mut prefix = prefix!($prefix);
            let (input, _) = prefix(input)?;
            let (input, val) = alphanumeric1(input)?;
            Ok((input, Self(val)))
        }
    };
}

trace_macros!(true);
#[derive(Debug, Clone, Copy, PartialEq)]
struct BirthYear(u32);
impl BirthYear {
    parse_num!("byr");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IssueYear(u32);
impl IssueYear {
    parse_num!("iyr");
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct ExpirationYear(u32);
impl ExpirationYear {
    parse_num!("eyr");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HeightUnit {
    Cm(u32),
    In(u32),
}

impl HeightUnit {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, val) = map_res(digit1, u32::from_str)(input)?;
        let (input, unit) = alt((tag("cm"), tag("in")))(input)?;
        match unit {
            "cm" => Ok((input, Self::Cm(val))),
            _ => Ok((input, Self::In(val))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Height(HeightUnit);

impl Height {
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut prefix = prefix!("hgt");
        let (input, _) = prefix(input)?;
        let (input, val) = HeightUnit::parse(input)?;
        Ok((input, Self(val)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct HairColor<'a>(&'a str);

impl<'a> HairColor<'a> {
    parse_str!("hcl");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct EyeColor<'a>(&'a str);

impl<'a> EyeColor<'a> {
    parse_str!("ecl");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PassportID(u32);

impl PassportID {
    parse_num!("pid");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CountryID(u32);

impl CountryID {
    parse_num!("cid");
}

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
            terminated(BirthYear::parse, space0),
            terminated(IssueYear::parse, space0),
            terminated(ExpirationYear::parse, space0),
            terminated(Height::parse, space0),
            terminated(HairColor::parse, space0),
            terminated(EyeColor::parse, space0),
            terminated(PassportID::parse, space0),
            terminated(CountryID::parse, space0),
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

#[test]
fn test_height_parse() {
    assert_eq!(
        Height::parse("hgt:172cm"),
        Ok(("", Height(HeightUnit::Cm(172))))
    );
}

#[test]
fn test_passport_parse() {
    let p =
        Passport::parse("hgt:172cm pid:170 hcl:17106b iyr:2012 ecl:gry cid:123 eyr:2020 byr:1990");
    assert_eq!(
        p,
        Passport {
            hgt: Some(Height(HeightUnit::Cm(172))),
            pid: Some(PassportID(170)),
            hcl: Some(HairColor("17106b")),
            iyr: Some(IssueYear(2012)),
            ecl: Some(EyeColor("gry")),
            cid: Some(CountryID(123)),
            eyr: Some(ExpirationYear(2020)),
            byr: Some(BirthYear(1990))
        }
    );
}

pub fn solve_a() -> io::Result<()> {
    let p =
        Passport::parse("hgt:172 pid:170 hcl:17106b iyr:2012 ecl:gry cid:123 eyr:2020 byr:1990");
    println!("{:?}", p);
    // Passport::parse("hgt:172in pid:170cm hcl:17106b iyr:2012 ecl:gry cid:123 eyr:2020 byr:1990");
    Ok(())
}
