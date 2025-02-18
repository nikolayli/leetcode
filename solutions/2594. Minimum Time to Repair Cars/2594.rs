// Time complexity: O(log⁡(min⁡(ranks)*cars^2))
// Space complexity: O(1)
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as i64;
        let mut l: i64 = 0;
        let mut r: i64 = *ranks.iter().min().unwrap() as i64 * cars * cars;

        while l < r {
            let m = l + (r - l) / 2;
            if Self::num_cars_fixed(&ranks, m) >= cars {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }

    fn num_cars_fixed(ranks: &Vec<i32>, minutes: i64) -> i64 {
        let mut cars_fixed = 0;

        for &rank in ranks {
            cars_fixed += (minutes as f64 / rank as f64).sqrt() as i64;
        }

        cars_fixed
    }
}
