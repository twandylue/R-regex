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
struct FSM {
    cols: Vec<FsmColumn>,
}

impl FSM {
    fn new() -> Self {
        Self {
            cols: Vec::<FsmColumn>::new(),
        }
    }

    fn push(&mut self, column: FsmColumn) {
        self.cols.push(column);
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

    fn is_match(&self, input: &str) -> bool {
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

        if state < self.cols.len() {
            state = self.cols[state].ts[FSM_NEWLINE];
        }

        return state >= self.cols.len();
    }
}

fn main() {
    let mut fsm = FSM::new();

    // NOTE: Failed state
    fsm.push(FsmColumn::new());

    let events = vec!['a' as usize, 'b' as usize, 'c' as usize, FSM_NEWLINE];
    for event in events {
        let mut col = FsmColumn::new();
        col.ts[event] = fsm.cols.len() + 1;
        fsm.push(col);
    }

    // fsm.dump();

    let input = vec!["Hello", "abc"];
    for i in input {
        let result = fsm.is_match(i);
        println!("{:?} => {:?}", i, result);
    }
}
