#[derive(Debug)]
pub struct Cursor {
    idx: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn next(&mut self) {
        self.idx += 1;
    } 

    pub fn goto(&mut self, to: usize) {
        self.idx = to;
    }

    pub fn forward(&mut self, amount: usize) {
        self.idx += amount;
    }

    pub fn current(&self) -> usize {
        self.idx
    }
}
