// Time complexity: O(nlog(max(d)))
// Space complexity: O(1)
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if bloom_day.len() < (m * k) as usize {
            return -1;
        }

        let get_bouquet_count = |waiting_days: i32| -> i32 {
            bloom_day
                .iter()
                .fold((0, k), |(bouquet_count, required_flowers), &day| {
                    if day > waiting_days {
                        (bouquet_count, k)
                    } else {
                        let new_required_flowers = required_flowers - 1;
                        if new_required_flowers == 0 {
                            (bouquet_count + 1, k)
                        } else {
                            (bouquet_count, new_required_flowers)
                        }
                    }
                })
                .0
        };

        let mut l = *bloom_day.iter().min().unwrap();
        let mut r = *bloom_day.iter().max().unwrap();

        while l < r {
            let mid = l + (r - l) / 2;
            if get_bouquet_count(mid) >= m {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l
    }
}
