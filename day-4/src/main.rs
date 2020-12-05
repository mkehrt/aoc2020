use {std::{env, fs::read_to_string, path::Path}, scan_fmt::scan_fmt};

#[derive(Default)]
struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: i32,
    hcl: i32,
    ecl: i32,
    pid: i32,
    cid: i32,
}

impl Passport {
    fn increment_field_count(&mut self, string: &String) {
        match string.as_str() {
            "byr" => self.byr += 1,
            "iyr" => self.iyr += 1,
            "eyr" => self.eyr += 1,
            "hgt" => self.hgt += 1,
            "hcl" => self.hcl += 1,
            "ecl" => self.ecl += 1,
            "pid" => self.pid += 1,
            "cid" => self.cid += 1,
            _ => unreachable!(),
        }
    }

    fn is_valid(&self) -> bool {
        (self.byr == 1)
            && (self.iyr == 1)
            && (self.eyr == 1)
            && (self.hgt == 1)
            && (self.hcl == 1)
            && (self.ecl == 1)
            && (self.pid == 1)
            && (self.cid == 1 || self.cid == 0)
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
            let (field_name, _) = scan_fmt!(field, "{}:{}", String, String).unwrap();
            passport_validator.increment_field_count(&field_name);
        }
        if passport_validator.is_valid() {
            count += 1;
        }
    }

    println!("{}", count);
}
