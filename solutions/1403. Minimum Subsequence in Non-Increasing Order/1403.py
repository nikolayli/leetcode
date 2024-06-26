# Time complexity: O(nlogn)
# Space complexity: O(n)
class Solution:
  def minSubsequence(self, nums: List[int]) -> List[int]:
    ans = []
    maxHeap = [-num for num in nums]
    heapq.heapify(maxHeap)
    half = sum(nums) // 2

    while half >= 0:
      ans.append(-maxHeap[0])
      half += heapq.heappop(maxHeap)

    return ans