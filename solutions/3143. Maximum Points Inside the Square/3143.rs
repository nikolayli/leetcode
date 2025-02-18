// Time complexity: O(n)
// Space complexity: O(1)
use std::collections::HashMap;

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut second_min_size = f64::INFINITY;
        let mut min_sizes: HashMap<char, f64> = HashMap::new();

        for (point, c) in points.iter().zip(s.chars()) {
            let x = point[0] as f64;
            let y = point[1] as f64;
            let sz = x.abs().max(y.abs());
            match min_sizes.get(&c) {
                Some(&current_min) if sz < current_min => {
                    second_min_size = second_min_size.min(current_min);
                    min_sizes.insert(c, sz);
                }
                Some(&current_min) => {
                    second_min_size = second_min_size.min(sz);
                }
                None => {
                    min_sizes.insert(c, sz);
                }
            }
        }

        min_sizes
            .values()
            .filter(|&&sz| sz < second_min_size)
            .count() as i32
    }
}
