#![feature(const_generics)]
#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]

use num::Integer;

struct BoundedQueue<const S: usize> {
    values: [usize; S],
    start: usize,
    end: usize,
    count: usize
}

impl <const S: usize> BoundedQueue<S> {
    pub fn new() -> Self {
        BoundedQueue {values: [0; S], start: 0, end: 0, count: 0}
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn peek(&self) -> usize {
        self.values[self.start]
    }

    fn inc(&self, index: usize) -> usize {
        (index + 1).mod_floor(&(self.values.len()))
    }

    pub fn enqueue(&mut self, value: usize) {
        self.values[self.end] = value;
        self.end = self.inc(self.end);
        self.count += 1;
    }

    pub fn dequeue(&mut self) -> usize {
        let result = self.values[self.start];
        self.start = self.inc(self.start);
        self.count -= 1;
        result
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use crate::BoundedQueue;

    #[test]
    fn it_works() {
        let mut q = BoundedQueue::<4>::new();
        assert!(q.is_empty());

        for x in 11..15 {
            q.enqueue(x);
            assert!(!q.is_empty());
            assert_eq!(q.len(), x % 10);
            assert_eq!(q.peek(), 11);
        }

        for x in 11..15 {
            let old_len = q.len();
            let v = q.dequeue();
            assert_eq!(x, v);
            assert_eq!(old_len - 1, q.len());
        }

        q.enqueue(12);
        q.enqueue(1);
        assert_eq!(q.dequeue(), 12);
        for x in 2..5 {
            q.enqueue(x);
        }
        for x in 1..5 {
            assert_eq!(x, q.dequeue());
        }
    }
}
