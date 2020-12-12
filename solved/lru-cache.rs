use std::collections::HashMap;

struct Entry {
    prev: usize,
    next: usize,
    key: i32,
    val: i32,
}

struct LRUCache {
    query: HashMap<i32, usize>,
    entries: Vec<Entry>,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let mut entries = Vec::with_capacity(capacity + 1);
        entries.push(Entry { prev: 0, next: 0, key: 0, val: 0 });
        LRUCache {
            query: HashMap::with_capacity(capacity),
            entries,
            capacity,
        }
    }

    #[inline]
    fn remove(&mut self, e: usize) {
        let entries = &mut self.entries;
        let prev = entries[e].prev;
        entries[prev].next = entries[e].next;
        let next = entries[e].next;
        entries[next].prev = entries[e].prev;
    }

    #[inline]
    fn push_back(&mut self, e: usize) {
        let entries = &mut self.entries;
        entries[e].prev = entries[0].prev;
        entries[e].next = 0;
        let tail = entries[0].prev;
        entries[tail].next = e;
        entries[0].prev = e;
    }

    fn get(&mut self, key: i32) -> i32 {
        let p = match self.query.get(&key) {
            Some(v) => *v,
            None => return -1,
        };
        let val = self.entries[p].val;
        self.remove(p);
        self.push_back(p);
        val
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        let mut e = match self.query.get(&key) {
            Some(e) => *e,
            None => 0,
        };
        if e == 0 {
            if self.entries.len() > self.capacity {
                // evict
                e = self.entries[0].next;
                self.query.remove(&self.entries[e].key);
                // reuse
                self.remove(e);
            } else {
                e = self.entries.len();
                self.entries.push(Entry { prev: 0, next: 0, key: 0, val: 0 });
            }
            self.entries[e].key = key;
            self.query.insert(key, e);
        } else {
            self.remove(e);
        }
        self.entries[e].val = value;
        self.push_back(e);
    }
}
