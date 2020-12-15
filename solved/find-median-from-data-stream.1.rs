struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            low: BinaryHeap::new(),
            high: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.high.len() < self.low.len() {
            self.low.push(num);
            self.high.push(Reverse(self.low.pop().unwrap()));
        } else {
            self.high.push(Reverse(num));
            self.low.push(self.high.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.low.len() > self.high.len() {
            *self.low.peek().unwrap_or(&0) as f64
        } else {
            (*self.low.peek().unwrap() + self.high.peek().unwrap().0) as f64 / 2f64
        }
    }
}
