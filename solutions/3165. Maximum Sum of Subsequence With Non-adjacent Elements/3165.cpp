// Time complexity: O(n + q*sqrt(n))
// Space complexity: O(sqrt(n))
class Solution {
 public:
  int maximumSumSubsequence(vector<int>& nums,
                            const vector<vector<int>>& queries) {
    long long ans = 0;
    int partSize = max(8, static_cast<int>(sqrt(nums.size())));
    vector<array<int, 4>> parts;

    for (int i = 0; i < nums.size(); i += partSize)
      parts.push_back(calculateMaxNonAdjacentSum(nums, i, i + partSize));

    for (const auto& query : queries) {
      int partIndex = query[0] / partSize;
      nums[query[0]] = query[1];
      parts[partIndex] = calculateMaxNonAdjacentSum(nums, partIndex * partSize,
                                                    (partIndex + 1) * partSize);
      ans += combineParts(parts);
      ans %= kMOD;
    }

    return static_cast<int>(ans);
  }

 private:
  const int kMOD = 1'000'000'007;

  array<int, 4> calculateMaxNonAdjacentSum(vector<int>& nums, int start,
                                           int end) {
    int firstLast = 0;
    int firstSecondLast = 0;
    int secondLast = 0;
    int secondSecondLast = 0;

    for (int i = start; i < min(static_cast<int>(nums.size()), end); ++i) {
      firstLast =
          max(exchange(firstSecondLast, firstLast) + nums[i], firstLast);
      if (i > start)
        secondLast =
            max(exchange(secondSecondLast, secondLast) + nums[i], secondLast);
    }

    return {firstLast, firstSecondLast, secondLast, secondSecondLast};
  }

  int combineParts(vector<array<int, 4>>& parts) {
    int maxLast = 0;
    int maxSecondLast = 0;

    for (auto [firstLast, firstSecondLast, secondLast, secondSecondLast] :
         parts) {
      int temp = maxLast;
      maxLast = max(maxLast + secondLast, maxSecondLast + firstLast);
      maxSecondLast =
          max(temp + secondSecondLast, maxSecondLast + firstSecondLast);
    }

    return maxLast;
  }
};
