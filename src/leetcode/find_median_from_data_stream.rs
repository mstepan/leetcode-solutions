use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * 295. Find Median from Data Stream
 * https://leetcode.com/problems/find-median-from-data-stream/
 *
 */
pub struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    pub fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    /**
     * time: O(lgN)
     */
    pub fn add_num(&mut self, val: i32) {
        if self.max_heap.is_empty() {
            self.min_heap.push(Reverse(val));
        } else {
            let last_from_max = self.max_heap.peek().unwrap();

            if val <= *last_from_max {
                self.max_heap.push(val);
            } else {
                self.min_heap.push(Reverse(val));
            }
        }

        let size_diff = (self.max_heap.len() as i32 - self.min_heap.len() as i32).abs();

        if size_diff >= 2 {
            self.rebalance();
        }
    }

    /**
     * time: O(1)
     */
    fn rebalance(&mut self) {
        if self.max_heap.len() > self.min_heap.len() {
            let last_max = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(last_max));
        } else {
            let last_min = self.min_heap.pop().unwrap();
            self.max_heap.push(last_min.0);
        }
    }

    /**
     * time: O(1)
     */
    pub fn find_median(&self) -> f64 {
        match self.max_heap.len().cmp(&self.min_heap.len()) {
            Ordering::Greater => *self.max_heap.peek().unwrap() as f64,
            Ordering::Less => self.min_heap.peek().unwrap().0 as f64,
            Ordering::Equal => {
                let max_val = *self.max_heap.peek().unwrap() as f64;
                let min_val = self.min_heap.peek().unwrap().0 as f64;
                (max_val + min_val) / 2.0
            }
        }
    }
}
