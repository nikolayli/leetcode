# Time complexity: O(n^2)
# Space complexity: O(n)
class Solution:
  def minimumBeautifulSubstrings(self, s: str) -> int:
    n = len(s)
    dp = [0] + [n + 1] * n

    for i in range(1, n + 1):
      if s[i - 1] == '0':
        continue
      num = 0
      for j in range(i, n + 1):
        num = (num << 1) + int(s[j - 1])
        if self._isPowerOfFive(num):
          dp[j] = min(dp[j], dp[i - 1] + 1)

    return -1 if dp[n] == n + 1 else dp[n]

  def _isPowerOfFive(self, num: int) -> bool:
    while num % 5 == 0:
      num //= 5

    return num == 1