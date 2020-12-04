use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut count = 0;
    let mut x = 0;
    for line in lines {
        let chars: Vec<char> = line.unwrap().chars().collect();
        let width = chars.len();
        let char = chars[x % width];
        if char.to_string() == "#" {
            count += 1;
        }
        x += 3;
    }
    println!("{}", count);
}
