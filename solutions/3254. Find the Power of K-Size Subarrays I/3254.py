# Time complexity: O(n)
# Space complexity: O(n)
class Solution:
  def resultsArray(self, nums: List[int], k: int) -> List[int]:
    ans = []
    start = 0

    for i, num in enumerate(nums):
      if i > 0 and num != nums[i - 1] + 1:
        start = i
      if i >= k - 1:
        ans.append(num if i - start + 1 >= k else -1)

    return ans