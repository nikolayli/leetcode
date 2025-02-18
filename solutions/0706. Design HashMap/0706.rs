// Time complexity: O(1)
// Space complexity: O(n)
const K_SIZE: usize = 10_usize.pow(6) + 1;

struct MyHashMap {
    lists: Box<[Option<i32>; K_SIZE]>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            lists: vec![None; K_SIZE].into_boxed_slice().try_into().unwrap(),
        }
    }

    fn put(&mut self, key: i32, valueue: i32) {
        if let Some(value) = self.lists.get_mut(key as usize) {
            *value = Some(valueue);
        }
    }

    fn get(&self, key: i32) -> i32 {
        self.lists
            .get(key as usize)
            .unwrap_or(&Some(-1))
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        if let Some(value) = self.lists.get_mut(key as usize) {
            *value = None;
        }
    }
}
