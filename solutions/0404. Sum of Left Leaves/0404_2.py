# Time complexity: O(n)
# Space complexity: O(h)
class Solution:
  def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
    if not root:
      return 0
    
    ans = 0
    stack = [root]

    while stack:
      root = stack.pop()
      if root.left:
        if not root.left.left and not root.left.right:
          ans += root.left.val
        else:
          stack.append(root.left)
      if root.right:
        stack.append(root.right)

    return ans          