// Time Complexity: O(log⁡n)
// Space Complexity: O(1)
class Solution {
 public:
  int binaryGap(int n) {
    int ans = 0;

    for (int d = -32; n; n /= 2, ++d)
      if (n % 2 == 1) {
        ans = max(ans, d);
        d = 0;
      }

    return ans;
  }
};