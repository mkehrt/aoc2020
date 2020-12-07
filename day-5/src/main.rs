// Credit to someone on The Digital World slack, probably Jake Kaufman, for
// unintentionally revealing the simple solution to this to me.

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
    let mut seats = Vec::new();
    let lines = reader.lines().map(|l| l.unwrap());
    for line in lines {
        let mut id = 0;
        let chars = line.chars();
        for char in chars {
            id *= 2;
            if (char == 'F') || (char == 'L') {
                // Nothing
            } else if (char == 'B') || (char == 'R') {
                id += 1;
            }
        }
        seats.push(id);
    }
    seats.sort();
    let pairs = seats.windows(2);
    for pair in pairs {
        if let &[low, high] = pair {
            if high != low + 1 {
                println!("{}", low + 1);
                break;
            }
        }
    }
}
