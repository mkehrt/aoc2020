use {
    scan_fmt::scan_fmt,
    std::{
        env,
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    },
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut count = 0;
    for line in lines {
        let (lower, upper, char, pwd) =
            scan_fmt!(&line.unwrap(), "{}-{} {}: {}", i32, i32, String, String).unwrap();
        let mut char_count = 0;
        for pwd_char in pwd.chars() {
            if pwd_char.to_string() == char {
                char_count += 1;
            }
        }
        if lower <= char_count && char_count <= upper {
            count += 1;
        }
    }
    println!("{}", count);
}
