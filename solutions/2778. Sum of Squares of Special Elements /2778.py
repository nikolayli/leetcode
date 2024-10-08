# Time complexity: O(n)
# Space complexity: O(1)
class Solution:
  def sumOfSquares(self, nums: List[int]) -> int:
    return sum(num**2 for i, num in enumerate(nums) 
               if len(nums) % (i + 1) == 0)