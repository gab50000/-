use std::{io, str::FromStr};

use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_till},
    character::complete::{anychar, char, digit1, line_ending, space1},
    combinator::{map_res, opt},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::{terminated, tuple},
    Err, IResult, InputTakeAtPosition,
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

fn space_or_newline(input: &str) -> IResult<&str, &str> {
    space1(input)
}

#[test]
fn test_space_or_newline() {
    assert_eq!(space_or_newline(" "), Ok(("", " ")));
    assert_eq!(space_or_newline("\n"), Ok(("", "\n")));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BirthYear(u32);
impl BirthYear {
    parse_num!("byr");
}

#[test]
fn test_birth_year() {
    assert_eq!(BirthYear::parse("byr:1234"), Ok(("", BirthYear(1234))));
    assert_eq!(BirthYear::parse("byr:1234 "), Ok((" ", BirthYear(1234))));
    assert!(BirthYear::parse(" byr:1234").is_err());
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

#[test]
fn test_height_parse() {
    assert_eq!(
        Height::parse("hgt:172cm"),
        Ok(("", Height(HeightUnit::Cm(172))))
    );
    assert_eq!(
        Height::parse("hgt:42in"),
        Ok(("", Height(HeightUnit::In(42))))
    );
}

#[derive(Debug, Clone, PartialEq)]
struct HairColor(String);

impl HairColor {
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut prefix = prefix!("hcl");
        let (input, _) = prefix(input)?;
        let (input, val) = till_whitespace(input)?;
        let val = String::from_str(val).unwrap();
        Ok((input, Self(val)))
    }
}

#[test]
fn test_hair_color() {
    assert_eq!(
        HairColor::parse("hcl:#abc"),
        Ok(("", HairColor("#abc".to_string())))
    );
    assert_eq!(
        HairColor::parse("hcl:gry"),
        Ok(("", HairColor("gry".to_string())))
    );
    assert_eq!(
        HairColor::parse("hcl:17106b"),
        Ok(("", HairColor("17106b".to_string())))
    );
}

#[derive(Debug, Clone, PartialEq)]
struct EyeColor(String);

impl EyeColor {
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut prefix = prefix!("ecl");
        let (input, _) = prefix(input)?;
        let (input, val) = till_whitespace(input)?;
        Ok((input, Self(String::from_str(val).unwrap())))
    }
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

#[derive(Debug, Clone, PartialEq)]
struct Passport {
    pub byr: Option<BirthYear>,
    pub iyr: Option<IssueYear>,
    pub eyr: Option<ExpirationYear>,
    pub hgt: Option<Height>,
    pub hcl: Option<HairColor>,
    pub ecl: Option<EyeColor>,
    pub pid: Option<PassportID>,
    pub cid: Option<CountryID>,
}

impl Passport {
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

    fn parse(input: &str) -> Self {
        let mut p = permutation((
            opt(terminated(BirthYear::parse, space_or_newline)),
            opt(terminated(IssueYear::parse, space_or_newline)),
            opt(terminated(ExpirationYear::parse, space_or_newline)),
            opt(terminated(Height::parse, space_or_newline)),
            opt(terminated(HairColor::parse, space_or_newline)),
            opt(terminated(EyeColor::parse, space_or_newline)),
            opt(terminated(PassportID::parse, space_or_newline)),
            opt(terminated(CountryID::parse, space_or_newline)),
        ));
        let result = p(input);

        let mut passport = Passport::new();
        if let Ok((_, values)) = result {
            passport.byr = values.0;
            passport.iyr = values.1;
            passport.eyr = values.2;
            passport.hgt = values.3;
            passport.hcl = values.4;
            passport.ecl = values.5;
            passport.pid = values.6;
            passport.cid = values.7;
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
fn test_passport_parse() {
    // let p = Passport::parse("hgt:172cm pid:170 hcl:17106b ");
    // let p = Passport::parse("hcl:gry ecl:172in ");
    let p =
        Passport::parse("hgt:172cm pid:170 hcl:17106b iyr:2012 ecl:gry cid:123 eyr:2020 byr:1990 ");
    // Passport::parse("iyr:2012 ecl:gry cid:123 eyr:2020 byr:1990 ");
    // Passport::parse("byr:1990 eyr:2020 hgt:172cm ");
    assert_eq!(
        p,
        Passport {
            hgt: Some(Height(HeightUnit::Cm(172))),
            pid: Some(PassportID(170)),
            hcl: Some(HairColor("17106b".to_string())),
            iyr: Some(IssueYear(2012)),
            ecl: Some(EyeColor("gry".to_string())),
            cid: Some(CountryID(123)),
            eyr: Some(ExpirationYear(2020)),
            byr: Some(BirthYear(1990))
        }
    );
}

#[test]
fn test_passport_parse2() {
    let input = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm
";
    assert_eq!(
        Passport::parse(input),
        Passport {
            hgt: Some(Height(HeightUnit::Cm(179))),
            pid: Some(PassportID(760753108)),
            hcl: Some(HairColor("#ae17e1".to_string())),
            iyr: Some(IssueYear(2013)),
            ecl: Some(EyeColor("brn".to_string())),
            cid: Some(CountryID(339)),
            eyr: Some(ExpirationYear(2024)),
            byr: Some(BirthYear(1931))
        }
    );
}

pub fn solve_a() -> io::Result<()> {
    let result = include_str!("../data/04.txt")
        .split("\n\n")
        .map(Passport::parse);

    for pp in result {
        println!("{:?}", pp);
    }
    Ok(())
}

fn till_whitespace(input: &str) -> IResult<&str, &str> {
    take_till(char::is_whitespace)(input)
}
#[test]
fn test_take_till() {
    assert_eq!(till_whitespace("bla "), Ok((" ", "bla")));
    assert_eq!(till_whitespace("bla"), Ok(("", "bla")));
}
