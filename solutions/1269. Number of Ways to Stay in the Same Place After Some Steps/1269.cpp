// Time complexity: O(n*min(n,m))
// Space complexity: O(n*min(n,m))
class Solution {
 public:
  int numWays(int steps, int arrLen) {
    constexpr int kMod = 1'000'000'007;
    const int n = min(arrLen, steps / 2 + 1);
    vector<long> dp(n);
    dp[0] = 1;

    while (steps-- > 0) {
      vector<long> newDp(n);
      for (int i = 0; i < n; ++i) {
        newDp[i] = dp[i];
        if (i - 1 >= 0)
          newDp[i] += dp[i - 1];
        if (i + 1 < n)
          newDp[i] += dp[i + 1];
        newDp[i] %= kMod;
      }
      dp = move(newDp);
    }

    return dp[0];
  }
};