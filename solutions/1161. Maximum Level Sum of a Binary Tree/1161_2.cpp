// Time complexity: O(n)
// Space complexity: O(n)
class Solution {
 public:
  int maxLevelSum(TreeNode* root) {
    vector<int> sumOfNodesAtLevel;
    dfs(root, 0, sumOfNodesAtLevel);

    int maxSum = INT_MIN;
    int result = 0;

    for (int i = 0; i < sumOfNodesAtLevel.size(); i++)
      if (maxSum < sumOfNodesAtLevel[i]) {
        maxSum = sumOfNodesAtLevel[i];
        result = i + 1;
      }
    
    return result;
  }
 private:
  void dfs(TreeNode* node, int level, vector<int>& sumOfNodesAtLevel) {
    if (node == nullptr)
      return;

    if (sumOfNodesAtLevel.size() == level)
      sumOfNodesAtLevel.push_back(node->val);
    else
      sumOfNodesAtLevel[level] += node->val;

    dfs(node->left, level + 1, sumOfNodesAtLevel);
    dfs(node->right, level + 1, sumOfNodesAtLevel);
  }
};