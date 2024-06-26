// Time complexity: O(n^2)
// Space complexity: O(n)
class Solution {
 public:
  int longestArithSeqLength(const vector<int>& nums) {
    const int n = nums.size();
    int ans = 0;
    vector<vector<int>> dp(n, vector<int>(1001));

    for (int i = 0; i < n; ++i)
      for (int j = 0; j < i; ++j) {
        const int k = nums[i] - nums[j] + 500;
        dp[i][k] = max(2, dp[j][k] + 1);
        ans = max(ans, dp[i][k]);
      }

    return ans;
  }
};