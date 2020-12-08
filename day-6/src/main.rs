use std::{cmp::Eq, collections::HashSet, env, fs::read_to_string, hash::Hash, path::Path};

fn optional_intersection<T: Eq + Hash + Copy>(
    optional_left: Option<HashSet<T>>,
    right: HashSet<T>,
) -> Option<HashSet<T>> {
    optional_left
        .map(|left| left.intersection(&right).map(|x| *x).collect())
        .or(Some(right))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let contents = read_to_string(path).unwrap();

    let groups = contents.split("\n\n");
    let mut total = 0;

    for group in groups {
        println!("Group: {}", group);
        let mut common_chars: Option<HashSet<char>> = None;
        // This only works if the file is newline terminated for some reason...
        let lines = group.split("\n");
        for line in lines {
            let mut chars = HashSet::new();
            for char in line.chars() {
                chars.insert(char);
            }
            common_chars = optional_intersection(common_chars, chars);
            println!("Common: {:?}", common_chars.clone().unwrap());
        }
        println!("");
        total += common_chars.unwrap().len();
    }

    println!("{}", total);
}
