# Time complexity: O(logn)
# Space complexity: O(1)
class Solution:

    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False

        y = x
        rev = 0

        while y:
            rev = rev * 10 + y % 10
            y = y // 10

        return rev == x
