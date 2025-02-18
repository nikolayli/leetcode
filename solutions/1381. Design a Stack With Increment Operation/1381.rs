// Time complexity: O(1)
// Space complexity: O(n)
struct CustomStack {
    stack_array: Vec<i32>,
    increment_array: Vec<i32>,
    top_index: isize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        CustomStack {
            stack_array: vec![0; max_size as usize],
            increment_array: vec![0; max_size as usize],
            top_index: -1,
        }
    }

    fn push(&mut self, x: i32) {
        if self.top_index < (self.stack_array.len() as isize) - 1 {
            self.top_index += 1;
            self.stack_array[self.top_index as usize] = x;
        }
    }

    fn pop(&mut self) -> i32 {
        if self.top_index < 0 {
            return -1;
        }
        let idx = self.top_index as usize;
        let result = self.stack_array[idx] + self.increment_array[idx];

        if self.top_index > 0 {
            self.increment_array[idx - 1] += self.increment_array[idx];
        }
        self.increment_array[idx] = 0;
        self.top_index -= 1;

        result
    }

    fn increment(&mut self, k: i32, val: i32) {
        if self.top_index >= 0 {
            let increment_index = (self.top_index as usize).min(k as usize - 1);
            self.increment_array[increment_index] += val;
        }
    }
}
