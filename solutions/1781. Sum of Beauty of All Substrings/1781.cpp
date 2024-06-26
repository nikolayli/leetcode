// Time complexity: O(n^2)
// Space complexity: O(1)
class Solution {
 public:
  int beautySum(string_view s) {
    int ans = 0;

    for (int i = 0; i < s.length(); ++i) {
      vector<int> count(26);
      for (int j = i; j < s.length(); ++j) {
        ++count[s[j] - 'a'];
        ans += ranges::max(count) - getMinFreq(count);
      }
    }

    return ans;
  }
 private:
  int getMinFreq(const vector<int>& count) {
    int minFreq = INT_MAX;
    for (const int freq : count)
      if (freq > 0)
        minFreq = min(minFreq, freq);

    return minFreq;
  }
};