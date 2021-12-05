from typing import List, Tuple
# traverse universe, DFS
# start from topleft, traverse until there is connected islands,
# then move to next grid that is not explored yet

# time complexity: O(1) for each grid action, traverse n grid, thus O(n)
# space complexity: O(n) explored status grid needed(using hash)

# Psuedocode
# explored = set<(i,j)>
# islands = 0
# for i,j in grid
#  - skip is (i,j) is explored
#  - mark (i,j) as explored
#  - next if i,j is water
#  - if i,j is land,
#   - recurse(right of i,j)
#   - recurse(bottom of i,j)
#   - islands += 1
# return islands

# recurse
# return if self is explored
# mark self as explored
# return if self is water
# recurse (right of self)
# recurse (bottom of self)

class Solution:
    LAND = '1'
    WATER = '0'

    def numIslands(self, grid: List[List[str]]) -> int:
        n, m = len(grid), len(grid[0])
        self.explored = set()
        islands = 0

        def traverse(grid, position: tuple[int, int]):
            if position in self.explored:
                return
            self.explored.add(position)
            i, j = position
            if i < 0 or j < 0 or n <= i or m <= j or grid[i][j] == self.WATER:
                return

            traverse(grid, (i+1, j))
            traverse(grid, (i, j+1))
            traverse(grid, (i-1, j))
            traverse(grid, (i, j-1))

        for i in range(n):
            for j in range(m):
                if (i,j) not in self.explored and grid[i][j] == self.LAND:
                    traverse(grid, (i,j))
                    islands += 1
        return islands
