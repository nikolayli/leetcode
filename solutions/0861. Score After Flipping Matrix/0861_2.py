# Time Complexity: O(mn)
# Space Complexity: O(1)
class Solution:
  def matrixScore(self, grid: List[List[int]]) -> int:
    m = len(grid)
    n = len(grid[0])
    ans = m

    for j in range(1, n):
      onesCount = sum(grid[i][j] == grid[i][0] for i in range(m))
      ans = ans * 2 + max(onesCount, m - onesCount)

    return ans