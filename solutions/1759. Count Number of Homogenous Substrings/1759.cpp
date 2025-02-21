// Time complexity: O(n)
// Space complexity: O(1)
class Solution {
 public:
  int countHomogenous(string_view s) {
    constexpr int kMod = 1'000'000'007;
    int ans = 0;
    int count = 0;
    char currentChar = '@';

    for (const char c : s) {
      count = c == currentChar ? count + 1 : 1;
      currentChar = c;
      ans += count;
      ans %= kMod;
    }

    return ans;
  }
};