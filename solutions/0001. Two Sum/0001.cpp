// Time complexity: O(n)
// Space complexity: O(n)
class Solution {
 public:
  vector<int> twoSum(const vector<int>& nums, int target) {
    unordered_map<int, int> numToIndex;

    for (int i = 0; i < nums.size(); ++i) {
      auto it = numToIndex.find(target - nums[i]);
      if (it != numToIndex.end()) return {it->second, i};
      numToIndex[nums[i]] = i;
    }
    throw;
  }
};
