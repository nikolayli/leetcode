// Time complexity: O(n)
// Space complexity: O(1)
class Solution {
 public:
  int largestCombination(const vector<int>& candidates) {
    constexpr int kMaxBit = 24;
    int ans = 0;

    for (int i = 0; i < kMaxBit; ++i) {
      int count = 0;
      for (const int candidate : candidates)
        if (candidate >> i & 1)
          ++count;
      ans = max(ans, count);
    }

    return ans;
  }
};