use {
    regex::Regex,
    std::{
        collections::HashSet,
        env,
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    },
};

#[derive(Debug)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl Op {
    fn from_str(str: &str) -> Self {
        match str {
            "nop" => Self::Nop,
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            _ => panic!("Unknown op {:}", str),
        }
    }
}

#[derive(Debug)]
struct Instr {
    op: Op,
    arg: i64,
}

impl Instr {
    fn from_str(str: &String) -> Self {
        let re = Regex::new(r"([a-z]+) ([-+0-9]+)").unwrap();
        let captures = re.captures_iter(str).next().unwrap();
        let (op_str, arg_str) = (&captures[1], &captures[2]);
        let op = Op::from_str(op_str);
        let arg = arg_str.parse::<i64>().unwrap();
        Instr { op, arg }
    }

    fn execute(&self, pc: &mut i64, acc: &mut i64) -> () {
        match self.op {
            Op::Nop => *pc = *pc + 1,
            Op::Acc => {
                *acc = *acc + self.arg;
                *pc = *pc + 1;
            }
            Op::Jmp => *pc = *pc + self.arg,
        }
    }

    fn corrupt(&mut self) {
        match self.op {
            Op::Nop => self.op = Op::Jmp,
            Op::Jmp => self.op = Op::Nop,
            _ => (),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut instrs: Vec<Instr> = lines.map(|l| Instr::from_str(&l)).collect();

    let mut pc: i64;
    let mut acc: i64 = 0;
    let len = instrs.len() as i64;
    'corrupted: for i in 0..instrs.len() {
        instrs[i].corrupt();
        pc = 0;
        acc = 0;
        let mut seen = HashSet::new();
        'execute: while !seen.contains(&pc) {
            if pc == len {
                break 'corrupted;
            } else if pc > len {
                continue 'execute;
            }
            seen.insert(pc);
            let instr = &instrs[pc as usize];
            instr.execute(&mut pc, &mut acc);
        }
        instrs[i].corrupt();
    }

    println!("{:}", acc);
}
