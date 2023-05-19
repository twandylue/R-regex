type FsmIndex = usize;

struct FsmColumn {
    ts: [FsmIndex; 127],
}

impl FsmColumn {
    fn new() -> Self {
        Self { ts: [0; 127] }
    }

    fn fill_range() {
        todo!();
    }
}

fn main() {
    println!("Hello")
}
