# Time complexity: O(mn)
# Space complexity: O(m)
class Solution:
  def countPoints(
    self, 
    points: List[List[int]], 
    queries: List[List[int]]
  ) -> List[int]:
    ans = []

    for xj, yj, rj in queries:
      count = 0
      for xi, yi in points:
        if (xi - xj)**2 + (yi - yj)**2 <= rj**2:
          count += 1
      ans.append(count)

    return ans