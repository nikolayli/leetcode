#include <gtest/gtest.h>

#include "../src/solutions/s0001_two_sum.cpp"

TEST(TwoSumTest, TestCases) {
  Solution solution;
  std::vector<int> result;

  result = solution.twoSum({2, 7, 11, 15}, 9);
  EXPECT_EQ(result, std::vector<int>({0, 1}));

  result = solution.twoSum({3, 2, 4}, 6);
  EXPECT_EQ(result, std::vector<int>({1, 2}));

  result = solution.twoSum({3, 3}, 6);
  EXPECT_EQ(result, std::vector<int>({0, 1}));
}
