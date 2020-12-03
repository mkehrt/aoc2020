use std::{env, fs::File, path::Path, io::{BufRead, BufReader}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let contents: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let length = contents.len();
    'outer: for i in 0..length {
        for j in i..length {
          for k in j..length {
           let int: i32 = contents[i].parse::<i32>().unwrap();
           let jnt: i32 = contents[j].parse::<i32>().unwrap();
           let knt: i32 = contents[k].parse::<i32>().unwrap();
             if int + jnt + knt == 2020 {
                 let output = int * jnt * knt;
                 println!("{}", output);
                 break 'outer;
               }
           }
        }
    }
}
