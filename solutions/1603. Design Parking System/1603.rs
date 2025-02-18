// Time complexity: O(1)
// Space complexity: O(1)
struct ParkingSystem {
    count: [i32; 3],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            count: [big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let index = (car_type - 1) as usize;
        if self.count[index] > 0 {
            self.count[index] -= 1;
            true
        } else {
            false
        }
    }
}
