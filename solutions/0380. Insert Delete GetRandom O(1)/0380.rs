// Time complexity: O(1)
// Space complexity: O(n)
use rand::seq::SliceRandom;
use std::collections::HashMap;

struct RandomizedSet {
    val_to_index: HashMap<i32, usize>,
    vals: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            val_to_index: HashMap::new(),
            vals: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.val_to_index.contains_key(&val) {
            return false;
        }

        self.val_to_index.insert(val, self.vals.len());
        self.vals.push(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.val_to_index.remove(&val) {
            None => false,
            Some(i) => {
                self.vals.swap_remove(i);
                if i < self.vals.len() {
                    self.val_to_index.insert(self.vals[i], i);
                }
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        *self.vals.choose(&mut rand::thread_rng()).unwrap()
    }
}
