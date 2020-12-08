use std::{env, fs::read_to_string, path::Path, collections::HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let contents = read_to_string(path).unwrap();

    let groups = contents.split("\n\n");
    let mut total= 0;

    for group in groups {
        let mut chars = HashSet::new();
        for char in group.chars() {
            if char >= 'a' && char <= 'z' {
                chars.insert(char);
            }
        }
        total+= chars.len();
    }

    println!("{}",total);
}
