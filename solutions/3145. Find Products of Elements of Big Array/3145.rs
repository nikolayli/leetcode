// Time complexity: O(nlogk)
// Space complexity: O(n)
impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        let mut ans = Vec::new();

        for query in queries {
            let (a, b, mod_val) = (query[0], query[1], query[2]);
            let product = Self::mod_pow(
                2,
                Self::sum_powers_first_k_big_nums(b + 1) - Self::sum_powers_first_k_big_nums(a),
                mod_val,
            );
            ans.push(product as i32);
        }

        ans
    }

    fn sum_powers_first_k_big_nums(k: i64) -> i64 {
        let num = Self::first_number_having_sum_bits_till_greater_than(k);
        let mut sum_powers = Self::sum_powers_till(num - 1);
        let mut remaining_count = k - Self::sum_bits_till(num - 1);

        for power in 0..Self::bit_length(num) {
            if num >> power & 1 == 1 {
                sum_powers += power;
                remaining_count -= 1;
                if remaining_count == 0 {
                    break;
                }
            }
        }

        sum_powers
    }

    fn first_number_having_sum_bits_till_greater_than(k: i64) -> i64 {
        let (mut l, mut r) = (1, k);
        while l < r {
            let m = l + (r - l) / 2;
            if Self::sum_bits_till(m) < k {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l
    }

    fn sum_bits_till(x: i64) -> i64 {
        let mut sum_bits = 0;
        let mut power_of_two = 1;
        while power_of_two <= x {
            sum_bits += (x / (2 * power_of_two)) * power_of_two;
            sum_bits += 0.max(x % (2 * power_of_two) + 1 - power_of_two);
            power_of_two *= 2;
        }

        sum_bits
    }

    fn sum_powers_till(x: i64) -> i64 {
        let mut sum_powers = 0;
        let mut power_of_two = 1;
        for power in 0..Self::bit_length(x) {
            sum_powers += (x / (2 * power_of_two)) * power_of_two * power;
            sum_powers += 0.max(x % (2 * power_of_two) + 1 - power_of_two) * power;
            power_of_two *= 2;
        }

        sum_powers
    }

    fn mod_pow(mut x: i64, mut n: i64, mod_val: i64) -> i64 {
        if n == 0 {
            return 1 % mod_val;
        }

        let mut res = 1;
        x %= mod_val;
        while n > 0 {
            if n % 2 == 1 {
                res = res * x % mod_val;
            }
            x = x * x % mod_val;
            n >>= 1;
        }

        res
    }

    fn bit_length(x: i64) -> i64 {
        if x == 0 {
            0
        } else {
            64 - x.leading_zeros() as i64
        }
    }
}
