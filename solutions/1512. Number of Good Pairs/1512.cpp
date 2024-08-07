// Time complexity: O(n)
// Space complexity: O(1)
class Solution {
 public:
  int numIdenticalPairs(const vector<int>& nums) {
    int ans = 0;
    vector<int> count(101);

    for (const int num : nums)
      ans += count[num]++;

    return ans;
  }
};