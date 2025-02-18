// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let (wait, curr) = customers.iter().fold((0.0, 0.0), |(wait, curr), c| {
            let curr = f64::max(curr, 1.0 * c[0] as f64) + c[1] as f64;
            (wait + curr - c[0] as f64, curr)
        });

        wait / customers.len() as f64
    }
}
