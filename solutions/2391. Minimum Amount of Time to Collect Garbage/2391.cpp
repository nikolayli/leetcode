// Time complexity: O(n)
// Space complexity: O(n)
class Solution {
 public:
  int garbageCollection(
    const vector<string>& garbage, 
    const vector<int>& travel
  ) {
    vector<int> prefix(travel.size());
    partial_sum(travel.begin(), travel.end(), prefix.begin());
    const int timeM = getTime(garbage, prefix, 'M');
    const int timeP = getTime(garbage, prefix, 'P');
    const int timeG = getTime(garbage, prefix, 'G');

    return timeM + timeP + timeG;
  }

 private:
  int getTime(
    const vector<string>& garbage, 
    const vector<int>& prefix, 
    char c
  ) {
    int characterCount = 0;
    int lastIndex = -1;

    for (int i = 0; i < garbage.size(); ++i) {
      string_view s = garbage[i];
      if (ranges::any_of(s, [c](const char g) { return g == c; }))
        lastIndex = i;
      characterCount += ranges::count(s, c);
    }

    return characterCount + (lastIndex <= 0 ? 0 : prefix[lastIndex - 1]);
  }
};