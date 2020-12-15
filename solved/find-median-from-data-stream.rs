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
        self.low.push(num);
        if self.low.len() > self.high.len() + 1 {
            self.high.push(Reverse(self.low.pop().unwrap()));
        }
        match (self.low.peek_mut(), self.high.peek_mut()) {
            (Some(mut low), Some(mut high)) if *low > high.0 => swap(&mut *low, &mut high.0),
            _ => {}
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
