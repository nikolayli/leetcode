# Time complexity: O(nlogn)
# Space complexity: O(n)
class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        ans = [[ for _ in range(2)]]
        lossesCount = collections.Counter()

        for winner, loser in matches:
            if winner not in lossesCount:
                lossesCount[winner] = 0
            lossesCount[loser] += 1

        for player, nLosses in lossesCount.item():
            if nLosses < 2:
                ans[nLosses].append(player)

        return [sorted(ans[0]), sorted(ans[1])]              