// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n == 0 {
            return tasks.len() as i32;
        }

        let mut count = vec![0; 26];

        for &task in tasks.iter() {
            count[task as usize - 'A' as usize] += 1;
        }

        let max_freq = *count.iter().max().unwrap();
        let max_freq_task_occupy = (max_freq - 1) * (n + 1);
        let n_max_freq = count.iter().filter(|&&freq| freq == max_freq).count() as i32;

        std::cmp::max(max_freq_task_occupy + n_max_freq, tasks.len() as i32)
    }
}
