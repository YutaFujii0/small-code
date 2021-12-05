from typing import List

# Find islands using DFS
# Psuedocode
# explored
# maxsize = 0
# for i,j in grid
# - next if (i,j) is explored or water
# - mark (i,j) as explored
# - traverse
# - maxsize = max[maxsize, traverse]
# return maxsize

# def traverse(count, grid)
# t = traverse(0, top)
# r = traverse(0, right)
# b = traverse(0, bottom)
# l = traverse(0, left)
# return count + t + r + b + l

# Complexity
# - time complexity: O(grid_size)
# - space complexity: O(grid_size) for explored storing

class Solution:
    LAND = 1
    WATER = 0

    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        n, m = len(grid), len(grid[0])
        explored = set()
        max_size = 0

        def traverse(grid: List[int], position: tuple[int, int]) -> int:
            i, j = position
            if i < 0 or n <= i or j < 0 or m <= j or grid[i][j] == self.WATER:
                return 0

            if (i,j) in explored:
                return 0

            explored.add((i,j))

            t = traverse(grid, (i-1, j))
            r = traverse(grid, (i, j+1))
            b = traverse(grid, (i+1, j))
            l = traverse(grid, (i, j-1))
            return t + r + b + l + 1

        for i in range(n):
            for j in range(m):
                if (i,j) not in explored and grid[i][j] == self.LAND:
                    land_size = traverse(grid, (i, j))
                    max_size = max([max_size, land_size])

        return max_size
