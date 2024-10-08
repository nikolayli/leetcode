// Time complexity: O(n*2^n)
// Space complexity: O(n^2*2^n)
class Solution {
 public:
  vector<int> diffWaysToCompute(string expression) {
    return ways(expression, {});
  }

 private:
  vector<int> ways(const string& s, unordered_map<string, vector<int>>&& mem) {
    if (const auto it = mem.find(s); it != mem.cend())
      return it->second;

    vector<int> ans;

    for (int i = 0; i < s.length(); ++i)
      if (ispunct(s[i]))
        for (const int a : ways(s.substr(0, i), move(mem)))
          for (const int b : ways(s.substr(i + 1), move(mem)))
            if (s[i] == '+')
              ans.push_back(a + b);
            else if (s[i] == '-')
              ans.push_back(a - b);
            else
              ans.push_back(a * b);

    return mem[s] = (ans.empty() ? vector<int>{stoi(s)} : ans);
  }
};