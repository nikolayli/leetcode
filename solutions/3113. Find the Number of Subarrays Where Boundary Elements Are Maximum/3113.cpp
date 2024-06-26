// Time complexity: O(n)
// Space complexity: O(n)
class Solution {
 public:
  long long numberOfSubarrays(const vector<int>& nums) {
    long long ans = 0;
    vector<pair<int, int>> stack;

    for (const int num : nums) {
      while (!stack.empty() && stack.back().first < num)
        stack.pop_back();
      if (stack.empty() || stack.back().first != num)
        stack.emplace_back(num, 0);
      ans += ++ stack.back().second;    
    }

    return ans;
  }
};