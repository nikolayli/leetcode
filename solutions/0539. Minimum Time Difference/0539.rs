// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut ans = 24 * 60;
        let mut first = 24 * 60;
        let mut bucket = vec![false; 24 * 60];

        for time in &time_points {
            let num = Self::parse_time(time);
            first = first.min(num);
            if bucket[num] {
                return 0;
            }
            bucket[num] = true;
        }

        let mut prev = first;
        for i in (first + 1)..bucket.len() {
            if bucket[i] {
                ans = ans.min(i - prev);
                prev = i;
            }
        }

        ans = ans.min(24 * 60 - prev + first);
        ans as i32
    }

    fn parse_time(time: &str) -> usize {
        let parts: Vec<&str> = time.split(':').collect();
        let hours = parts[0].parse::<usize>().unwrap();
        let minutes = parts[1].parse::<usize>().unwrap();

        hours * 60 + minutes
    }
}
