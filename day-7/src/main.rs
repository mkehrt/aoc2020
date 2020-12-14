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
            let outer = Bag {
                count: 1,
                adj: outer_adj.clone(),
                color: outer_color.clone(),
            };
            println!("OUTER: {:?}, {:?}", outer_adj, outer_color);
            for captures in inner_re.captures_iter(inner_str) {
                let (count, adj, color) = (
                    captures[1].parse::<i64>().unwrap(),
                    captures[2].to_string(),
                    captures[3].to_string(),
                );
                println!("INNER: {:?}, {:?}", adj, color);
                let inner = Bag { count, adj, color };
                let mut inners = topology.get(&outer).unwrap_or(&HashSet::new()).clone();
                inners.insert(inner.clone());
                topology.insert(outer.clone(), inners);
            }
        }
    }

    println!("TOP: {:?}", topology);

    let mine = Bag {
        count: 1,
        adj: "shiny".to_string(),
        color: "gold".to_string(),
    };

    let total = total_bags(&topology, &mine);
    println!("{}", total - 1);
}

fn total_bags(topology: &HashMap<Bag, HashSet<Bag>>, outer: &Bag) -> i64 {
    let mut total: i64 = outer.count;

        let outer_key = Bag {
            count: 1,
            ..outer.clone()
        };


    let empty: HashSet<Bag> = HashSet::new();
    let inners = match topology.get(&outer_key) {
        Some(i) => i,
        None => &empty,
    };

    for inner in inners {
        total = total + (outer.count * total_bags(topology, &inner));
    }

    total
}
