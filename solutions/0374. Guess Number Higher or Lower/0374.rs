// Time complexity: O(logn)
// Space complexity: O(1)

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        while (l < r) {
            let m = l + (r - l) / 2;
            if (guess(m) <= 0) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}
