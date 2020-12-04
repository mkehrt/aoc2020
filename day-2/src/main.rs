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
            scan_fmt!(&line.unwrap(), "{}-{} {}: {}", usize, usize, String, String).unwrap();
        let chars: Vec<char> = pwd.chars().collect();
        if (chars[lower - 1].to_string() == char) ^ (chars[upper - 1].to_string() == char) {
            count += 1;
        }
    }
    println!("{}", count);
}
