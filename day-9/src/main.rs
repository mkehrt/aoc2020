use {
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
    let vals: Vec<i64> = reader.lines().map(|l| l.unwrap().parse::<i64>().unwrap()).collect();
    let windows = vals.windows(26);

    let mut foot: i64 = 0;
    'window: for window in windows {
      let body = &window[..25];
      foot = window[25];
      for i in 0..25 {
        for j in i..25 {
          if body[i] + body[j] == foot {
            continue 'window;
          }
        }
      }
      break 'window;
    }

    println!("{:}", foot);
}
