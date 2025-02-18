// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut s: Vec<char> = num.to_string().chars().collect();
        let mut last_index = vec![-1; 10];

        for (i, &c) in s.iter().enumerate() {
            last_index[c.to_digit(10).unwrap() as usize] = i as i32;
        }

        for (i, &c) in s.iter().enumerate() {
            for d in (c.to_digit(10).unwrap() + 1..=9).rev() {
                if last_index[d as usize] > i as i32 {
                    s.swap(i, last_index[d as usize] as usize);
                    return s.iter().collect::<String>().parse::<i32>().unwrap();
                }
            }
        }

        num
    }
}
