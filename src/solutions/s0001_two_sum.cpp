#include <vector>
#include <unordered_map>
#include <stdexcept>

class Solution {
 public:
  std::vector<int> twoSum(const std::vector<int>& nums, int target) {
    std::unordered_map<int, int> numToIndex;

    for (int i = 0; i < nums.size(); ++i) {
      auto it = numToIndex.find(target - nums[i]);
      if (it != numToIndex.end()) return {it->second, i};
      numToIndex[nums[i]] = i;
    }
    throw std::invalid_argument("No two sum solution");
  }
};
