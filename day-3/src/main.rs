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
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut product: i64 = 1;
    let length = lines.len();
    for (dx, dy) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;
        while y < length {
            let line = &lines[y];
            let chars: Vec<char> = line.chars().collect();
            let width = chars.len();
            let char = chars[x % width];
            if char.to_string() == "#" {
                count += 1;
            }
            x += dx;
            y += dy;
        }
        println!("{}", count);
        product *= count;
    }
    println!("{}", product);
}
