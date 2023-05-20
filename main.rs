use std::ops::Range;

type FSM_INDEX = usize;
const FSM_COL_SIZE: usize = 130;
const FSM_NEW_LINE: usize = 129;

#[derive(Debug)]
struct FsmColumn {
    ts: [FSM_INDEX; FSM_COL_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            ts: [0; FSM_COL_SIZE],
        }
    }

    fn fill_range(&mut self, range: Range<char>, state: FSM_INDEX) {
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
        for symbol in 0..FSM_COL_SIZE {
            print!("{:03} => ", symbol);
            for column in &self.cols {
                print!("{v} ", v = column.ts[symbol]);
            }
            println!();
        }
    }
}

fn main() {
    let mut fsm = FSM::new();

    // NOTE: Failed state
    fsm.push(FsmColumn::new());

    {
        let mut col = FsmColumn::new();
        // NOTE: 'a' is an event
        col.ts['a' as usize] = fsm.cols.len() + 1;
        fsm.push(col);
    }

    {
        let mut col = FsmColumn::new();
        col.ts['b' as usize] = fsm.cols.len() + 1;
        fsm.push(col);
    }

    {
        let mut col = FsmColumn::new();
        col.ts['c' as usize] = fsm.cols.len() + 1;
        fsm.push(col);
    }

    {
        let mut col = FsmColumn::new();
        col.ts[FSM_NEW_LINE] = fsm.cols.len() + 1;
        fsm.push(col);
    }

    fsm.dump();
}
