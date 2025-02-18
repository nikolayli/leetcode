// Time complexity: O(nlog(logn))
// Space complexity: O(n)
use std::cmp::Ordering;

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let k_max = 1000;
        let primes = Self::sieve_eratosthenes(k_max);

        let mut prev_num = 0;
        for mut num in nums {
            if let Ok(i) = primes.binary_search_by(|&p| {
                if p < num - prev_num {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }) {
                if i > 0 {
                    num -= primes[i - 1];
                }
            } else {
                let i = primes.partition_point(|&p| p < num - prev_num);
                if i > 0 {
                    num -= primes[i - 1];
                }
            }
            if num <= prev_num {
                return false;
            }
            prev_num = num;
        }

        true
    }

    fn sieve_eratosthenes(n: usize) -> Vec<i32> {
        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..((n as f64).sqrt() as usize + 1) {
            if is_prime[i] {
                for j in (i * i..n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        is_prime
            .iter()
            .enumerate()
            .filter_map(|(i, &prime)| if prime { Some(i as i32) } else { None })
            .collect()
    }
}
