struct MinStack {
    vec: Vec<(i32, i32)>,
    smallest: i32,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            smallest: i32::MAX,
        }
    }

    pub fn push(&mut self, val: i32) {
        self.vec.push((val, self.smallest));
        if val < self.smallest {
            self.smallest = val;
        }
    }

    pub fn pop(&mut self) {
        let (val, smallest) = self.vec.pop().unwrap();
        if val == self.smallest {
            self.smallest = smallest
        }
    }

    pub fn top(&self) -> i32 {
        self.vec.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.smallest
    }
}
