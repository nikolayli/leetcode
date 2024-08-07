// Time complexity: O(n)
// Space complexity: O(1)
class Solution {
 public:
  vector<int> nodesBetweenCriticalPoints(ListNode* head) {
    int minDistance = INT_MAX;
    int firstMaIndex = -1;
    int prevMaIndex = -1;
    int index = 1;
    ListNode* prev = head;
    ListNode* curr = head->next;

    while (curr->next) {
      if (curr->val > prev->val && curr->val > curr->next->val ||
          curr->val < prev->val && curr->val < curr->next->val) {
        if (firstMaIndex == -1)
          firstMaIndex = index;
        if (prevMaIndex != -1)
          minDistance = min(minDistance, index - prevMaIndex);
        prevMaIndex = index;
      }
      prev = curr;
      curr = curr->next;
      ++index;
    }

    if (minDistance == INT_MAX)
      return {-1, -1};
    return {minDistance, prevMaIndex - firstMaIndex};
  }
};