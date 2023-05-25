type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_ENDLINE: usize = 129;

#[derive(Default, Copy, Clone, Debug)]
struct FsmAction {
    next: FsmIndex,
    offset: i32,
}

#[derive(Debug, Copy, Clone)]
struct FsmColumn {
    ts: [FsmAction; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            ts: [FsmAction::default(); FSM_COLUMN_SIZE],
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
                '$' => {
                    col.ts[FSM_ENDLINE] = FsmAction {
                        next: fsm.cols.len() + 1,
                        offset: 1,
                    };
                    fsm.cols.push(col);
                }
                '.' => {
                    for i in 33..127 {
                        col.ts[i] = FsmAction {
                            next: fsm.cols.len() + 1,
                            offset: 1,
                        };
                    }
                    fsm.cols.push(col);
                }
                '+' => {
                    // NOTE: a+ => aa*
                    let n = fsm.cols.len();
                    fsm.cols.push(fsm.cols.last().unwrap().clone());
                    for t in fsm.cols.last_mut().unwrap().ts.iter_mut() {
                        if t.next == n {
                            // NOTE:
                        } else if t.next == 0 {
                            t.next = n + 1;
                            t.offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                '*' => {
                    let n = fsm.cols.len();
                    for t in fsm.cols.last_mut().unwrap().ts.iter_mut() {
                        // NOTE: back to previous char
                        if t.next == n {
                            t.next = n - 1;
                        } else if t.next == 0 {
                            t.next = n;
                            t.offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                _ => {
                    col.ts[c as usize] = FsmAction {
                        next: fsm.cols.len() + 1,
                        offset: 1,
                    };
                    fsm.cols.push(col);
                }
            }
        }

        fsm
    }

    fn dump(&self) {
        for symbol in 0..FSM_COLUMN_SIZE {
            print!("{:03} => ", symbol);
            for column in &self.cols {
                print!(
                    "({}, {}) ",
                    column.ts[symbol].next, column.ts[symbol].offset
                );
            }
            println!();
        }
    }

    fn match_str(&self, input: &str) -> bool {
        // NOTE:
        // Failed state: 0
        // Initial state: 1
        let mut state = 1;
        let mut head: usize = 0;
        let chars = input.chars().collect::<Vec<_>>();

        while 0 < state && state < self.cols.len() && head < chars.len() {
            let action = self.cols[state].ts[chars[head] as usize];
            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }

        if state == 0 {
            return false;
        }

        if state < self.cols.len() {
            let action = self.cols[state].ts[FSM_ENDLINE];
            state = action.next;
        }

        return state >= self.cols.len();
    }
}

fn main() {
    let pattern = "a*bc$";
    // BUG:
    // let pattern = ".*bc";
    let regex = Regex::compile(pattern);

    regex.dump();
    println!("------------------------");
    println!("Regex: '{pattern}'");

    let inputs = vec!["aaaaaaaaaaaabc", "bc"];
    for input in inputs {
        let result = regex.match_str(input);
        println!("{:?} => {:?}", input, result);
    }
}
