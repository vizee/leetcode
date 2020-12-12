impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::{BinaryHeap, HashMap};
        fn heap_top(heap: &mut BinaryHeap<i32>, counter: &HashMap<i32, i32>) -> i32 {
            while let Some(top) = heap.peek() {
                if counter[top] > 0 {
                    return *top;
                }
                heap.pop();
            }
            0
        }
        let mut heap = BinaryHeap::new();
        let mut counter = HashMap::new();

        let mut events = buildings
            .iter()
            .flat_map(|p| vec![(p[0], p[2]), (p[1], -p[2])])
            .collect::<Vec<_>>();
        events.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

        let mut res = Vec::new();
        for e in events {
            let (h, leave) = if e.1 < 0 { (-e.1, true) } else { (e.1, false) };
            if !leave {
                let max_h = heap_top(&mut heap, &mut counter);
                if h > max_h {
                    res.push(vec![e.0, h]);
                }
                let v = counter.entry(h).or_insert(0);
                *v += 1;
                if *v == 1 {
                    heap.push(h);
                }
            } else {
                let v = counter.get_mut(&h).unwrap();
                *v -= 1;
                let max_h = heap_top(&mut heap, &mut counter);
                if h > max_h {
                    res.push(vec![e.0, max_h]);
                }
            }
        }
        res
    }
}
