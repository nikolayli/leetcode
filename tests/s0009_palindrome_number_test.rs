use leetcode::solutions::s0009_palindrome_number::Solution;

#[test]
fn test_is_palindrome() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
}
