# Time Complexity: O(n)
# Space Complexity: O(n)
class Solution:
  def sumSubarrayMins(self, arr: List[int]) -> int:
    kMod = 1_000_000_007
    n = len(arr)
    ans = 0
    prevMin = [-1] * n
    nextMin = [n] * n
    stack = []

    for i, a in enumerate(arr):
      while stack and arr[stack[-1]] > a:
        index = stack.pop()
        nextMin[index] = i
      if stack:
        prevMin[i] = stack[-1]
      stack.append(i)

    for i, a in enumerate(arr):
      ans += a * (i - prevMin[i]) * (nextMin[i] - i)
      ans %= kMod

    return ans