// Time complexity: O(n)
// Space complexity: O(n)
class Solution {
    int minDifference = INT_MAX;
    TreeNode* prevNode;

    void inorderTraversal(TreeNode* node) {
        if (node == nullptr) {
            return;
        }

        inorderTraversal(node->left);
        if (prevNode != nullptr) {
            minDifference = min(minDifference, node->val - prevNode->val);
        }
        prevNode = node;
        inorderTraversal(node->right);
    }
public:
    int getMinimumDifference(TreeNode* root) {
        inorderTraversal(root);
        return minDifference;
    }
};