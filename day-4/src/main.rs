use {
    scan_fmt::scan_fmt,
    std::{env, fs::read_to_string, path::Path},
};

#[derive(Debug)]
enum Height {
    Inches(i32),
    Centimeters(i32),
}

impl Height {
    fn from_str(str: &str) -> Option<Self> {
        println!("Height str {}", str);
        if let Ok((value, unit)) = scan_fmt!(str, "{d}{}", i32, String) {
            match unit.as_str() {
                "in" => {
                    println!("Height inches {}", value);
                    Some(Height::Inches(value))
                }
                "cm" => {
                    println!("Height cms {}", value);
                    Some(Height::Centimeters(value))
                }
                _ => {
                    println!("Height no unit");
                    None
                }
            }
        } else {
            println!("Height no match");
            None
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            &Height::Inches(ins) => (59 <= ins) && (ins <= 76),
            &Height::Centimeters(cms) => (150 <= cms) && (cms <= 193),
        }
    }
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl EyeColor {
    fn from_str(str: &str) -> Option<Self> {
        match str {
            "amb" => Some(EyeColor::Amber),
            "blu" => Some(EyeColor::Blue),
            "brn" => Some(EyeColor::Brown),
            "gry" => Some(EyeColor::Gray),
            "grn" => Some(EyeColor::Green),
            "hzl" => Some(EyeColor::Hazel),
            "oth" => Some(EyeColor::Other),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<EyeColor>,
    pid: Option<i32>, // 0 padded
    cid: Option<String>,
}

impl Passport {
    fn update_field(&mut self, key: &String, value: &String) {
        match key.as_str() {
            "byr" => self.byr = Self::parse_k_digit_number(value, 4),
            "iyr" => self.iyr = Self::parse_k_digit_number(value, 4),
            "eyr" => self.eyr = Self::parse_k_digit_number(value, 4),
            "hgt" => self.hgt = Height::from_str(value),
            "hcl" => self.hcl = Self::parse_hex_color(value),
            "ecl" => self.ecl = EyeColor::from_str(value),
            "pid" => self.pid = Self::parse_k_digit_number(value, 9),
            "cid" => self.cid = Some(value.to_string()),
            _ => panic!("Unkown key {}", key),
        }
    }

    fn parse_k_digit_number(value: &String, k: i32) -> Option<i32> {
        let fmt_str = format!("{{/^\\d{{{}}}$/}}", k); // ugh
        let parsed = scan_fmt!(value, fmt_str.as_str(), i32).ok();
        parsed
    }

    fn parse_hex_color(value: &String) -> Option<String> {
        let parsed = scan_fmt!(value, r"#{/[0-9a-f]{6}/}", String).ok();
        parsed
    }

    fn validate_integer_range(lower: i32, higher: i32, x: i32) -> bool {
        (lower <= x) && (x <= higher)
    }

    fn is_valid(&self) -> bool {
        (self
            .byr
            .map(|x| Self::validate_integer_range(1920, 2002, x))
            .unwrap_or(false))
            && (self
                .iyr
                .map(|x| Self::validate_integer_range(2010, 2020, x))
                .unwrap_or(false))
            && (self
                .eyr
                .map(|x| Self::validate_integer_range(2020, 2030, x))
                .unwrap_or(false))
            && (self.hgt.as_ref().map(|x| x.is_valid()).unwrap_or(false))
            && (self.hcl.is_some())
            && (self.ecl.is_some())
            && (self.pid.is_some())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let contents = read_to_string(path).unwrap();

    let passports = contents.split("\n\n");
    let mut count = 0;

    for passport in passports {
        let fields = passport.split_whitespace();
        let mut passport_validator = Passport::default();
        for field in fields {
            let (key, value) = scan_fmt!(field, "{}:{}", String, String).unwrap();
            passport_validator.update_field(&key, &value);
        }
        println!(
            "{}\n{:?}\n{}\n",
            passport,
            passport_validator,
            passport_validator.is_valid()
        );

        if passport_validator.is_valid() {
            count += 1;
        }
    }

    println!("{}", count);
}
