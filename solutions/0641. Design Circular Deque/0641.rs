// Time complexity: O(1)
// Space complexity: O(k)
struct MyCircularDeque {
    k: usize,
    q: Vec<i32>,
    size: usize,
    front: usize,
    rear: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let k = k as usize;
        MyCircularDeque {
            k,
            q: vec![0; k],
            size: 0,
            front: 0,
            rear: k - 1,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.k - 1) % self.k;
        self.q[self.front] = value;
        self.size += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.rear = (self.rear + 1) % self.k;
        self.q[self.rear] = value;
        self.size += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.k;
        self.size -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear + self.k - 1) % self.k;
        self.size -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.front]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.rear]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.k
    }
}
