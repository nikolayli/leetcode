// Time complexity: O(1)
// Space complexity: O(k)
use std::collections::VecDeque;

struct DataStream {
    value: i32,
    k: usize,
    q: VecDeque<i32>,
    count: usize,
}

impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self {
            value,
            k: k as usize,
            q: VecDeque::with_capacity(k as usize),
            count: 0,
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        if self.q.len() == self.k {
            if let Some(front) = self.q.pop_front() {
                if front == self.value {
                    self.count -= 1;
                }
            }
        }
        if num == self.value {
            self.count += 1;
        }
        self.q.push_back(num);
        self.count == self.k
    }
}
