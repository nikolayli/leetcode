// Time complexity: O(mn^2)
// Space complexity: O(m)
class Solution {
 public:
  int numSubmatrixSumTarget(const vector<vector<int>>& matrix, int target) {
    const int m = matrix.size();
    const int n = matrix[0].size();
    int ans = 0;

    for (auto& row : matrix)
      for (int i = 1; i < n; ++i)
        row[i] += row[i - 1];

    for (int baseCol = 0; baseCol < n; ++baseCol)
      for (int j = baseCol; j < n; ++j) {
        unordered_map<int, int> prefixCount{{0, 1}};
        int sum = 0;
        for (int i = 0; i < m; ++i) {
          if (baseCol > 0)
            sum -= matrix[i][baseCol - 1];
          sum += matrix[i][j];
          if (const auto it = prefixCount.find(sum - target);
              it != prefixCount.cend())
            ans += it->second;
          ++prefixCount[sum];
        }
      }

    return ans;
  }
};