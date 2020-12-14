use {
    regex::Regex,
    std::{
        collections::{HashMap, HashSet},
        env,
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    },
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Bag {
    count: i64,
    adj: String,
    color: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut topology = HashMap::new();
    let outer_re = Regex::new(r"([a-z]+) ([a-z]+) bag").unwrap();
    let inner_re = Regex::new(r"(\d+) ([a-z]+) ([a-z]+) bag").unwrap();
    for line in lines {
        println!("LINE: {:?}", line);
        if let [outer_str, inner_str] = line.split("contain").collect::<Vec<&str>>()[..] {
            let captures = outer_re.captures_iter(outer_str).next().unwrap();
            let (outer_adj, outer_color) = (captures[1].to_string(), captures[2].to_string());
            println!("OUTER: {:?}, {:?}", outer_adj, outer_color);
            for captures in inner_re.captures_iter(inner_str) {
                let (count, adj, color) = (
                    captures[1].parse::<i64>().unwrap(),
                    captures[2].to_string(),
                    captures[3].to_string(),
                );
                println!("INNER: {:?}, {:?}", adj, color);
                let inner = Bag { count: 1, adj, color };
                let outer = Bag {
                    count,
                    adj: outer_adj.clone(),
                    color: outer_color.clone(),
                };
                let mut outers = topology.get(&inner).unwrap_or(&HashSet::new()).clone();
                outers.insert(outer.clone());
                topology.insert(inner, outers);
            }
        }
    }

    println!("TOP: {:?}", topology);

    let mut seen = HashSet::new();
    let mine = Bag {
        count: 1,
        adj: "shiny".to_string(),
        color: "gold".to_string(),
    };
    let mut todo = vec![mine];

    while !todo.is_empty() {
        println!("TODO: {:?}", todo);
        let inner = todo.pop().unwrap();
        let outers = match topology.get(&inner) {
            Some(o) => o,
            None => continue,
        };
        for outer in outers {
            let outer = Bag {
                count: 1,
                ..outer.clone()
            };
            if !seen.contains(&outer) {
                seen.insert(outer.clone());
                todo.push(outer.clone());
            }
        }
    }

    println!("{}", seen.len());
}
