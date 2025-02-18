// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        if (num.len() == k) {
            "0".to_string();
        }

        let mut ans = String::with_capacity(num.len());

        for c in num.chars() {
            while k > 0 && !ans.is_empty() && ans.chars().last().unwrap() > c {
                ans.pop();
                k -= 1;
            }
            if ans.is_empty() && c == '0' {
                continue;
            }
            ans.push(c);
        }

        while k > 0 {
            ans.pop();
            k -= 1;
        }

        if ans.is_empty() {
            String::from("0")
        } else {
            ans
        }
    }
}
