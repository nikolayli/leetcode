# Time complexity: O(nlogn)
# Space complexity: O(n)
class Solution:
  def maxPalindromesAfterOperations(self, words: List[str]) -> int:
    ans = 0
    count = collections.Counter(''.join(words))
    pairs = sum(value // 2 for value in count.values())

    for length in sorted(len(word) for word in words):
      needPairs = length // 2

      if pairs < needPairs:
        return ans

      ans += 1
      pairs -= needPairs

    return ans