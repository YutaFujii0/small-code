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
        self.explored = set()
        islands = 0
        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if (i,j) not in self.explored and grid[i][j] == self.LAND:
                    self.traverse(grid, (i,j))
                    islands += 1
        return islands

    def traverse(self, grid, position: Tuple(int, int)):
        if position in self.explored:
            return
        self.explored.add(position)
        i, j = position
        if i < 0 or j < 0 or len(grid) <= i or len(grid[0]) <= j or grid[i][j] == self.WATER:
            return

        self.traverse(grid, (i+1, j))
        self.traverse(grid, (i, j+1))
        self.traverse(grid, (i-1, j))
        self.traverse(grid, (i, j-1))
