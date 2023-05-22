use std::ops::Range;

type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_NEWLINE: usize = 129;

#[derive(Debug)]
struct FsmColumn {
    ts: [FsmIndex; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            ts: [0; FSM_COLUMN_SIZE],
        }
    }

    fn fill_range(&mut self, range: Range<char>, state: FsmIndex) {
        for c in range {
            self.ts[c as usize] = state;
        }
    }
}

#[derive(Debug)]
struct Regex {
    cols: Vec<FsmColumn>,
}

impl Regex {
    fn compile(src: &str) -> Self {
        let mut fsm = Regex { cols: Vec::new() };

        // NOTE: Initial state: 1(fsm.cols.len() = 1)
        fsm.cols.push(FsmColumn::new());

        for c in src.chars() {
            let mut col = FsmColumn::new();

            match c {
                '$' => col.ts[FSM_NEWLINE] = fsm.cols.len() + 1,
                _ => col.ts[c as usize] = fsm.cols.len() + 1,
            }
            fsm.cols.push(col);
        }

        fsm
    }

    fn dump(&self) {
        for symbol in 0..FSM_COLUMN_SIZE {
            print!("{:03} => ", symbol);
            for column in &self.cols {
                print!("{v} ", v = column.ts[symbol]);
            }
            println!();
        }
    }

    fn match_str(&self, input: &str) -> bool {
        // NOTE:
        // Failed state: 0
        // Initial state: 1
        let mut state = 1;
        for c in input.chars() {
            if state == 0 || state >= self.cols.len() {
                break;
            }
            state = self.cols[state].ts[c as usize];
        }

        if state == 0 {
            return false;
        }

        // NOTE: new line is not a character, it is end of line.
        if state < self.cols.len() {
            state = self.cols[state].ts[FSM_NEWLINE];
        }

        return state >= self.cols.len();
    }
}

fn main() {
    let regex = Regex::compile("abcdefghijk$");

    regex.dump();
    println!("------------------------");

    let inputs = vec!["Hello", "abc", "abcde", "abcdefghijk"];
    for input in inputs {
        let result = regex.match_str(input);
        println!("{:?} => {:?}", input, result);
    }
}
