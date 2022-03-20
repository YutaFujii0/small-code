from typing import List
# using hash map
# O(n) scan
# if any hits, it means sudoku is invalid
# # of hash maps: 18?
# 9 for row, 9 for column, 9 for sub-boxes
# index of sub-boxes
# 0 | 1 | 2
# 3 | ..
# ..    | 8

class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        rows = [set(),set(),set(),set(),set(),set(),set(),set(),set()]
        columns = [set(),set(),set(),set(),set(),set(),set(),set(),set()]
        subBoxes = [set(),set(),set(),set(),set(),set(),set(),set(),set()]
        for i, row in enumerate(board):
            for j, val in enumerate(row):
                if val == ".": continue
                if val in rows[i]:
                    print('row')
                    print(rows)
                    return False
                else:
                    rows[i].add(val)
                if val in columns[j]:
                    print('column')
                    return False
                else:
                    columns[j].add(val)
                if val in subBoxes[(i//3)*3 + (j//3)]:
                    return False
                else:
                    subBoxes[(i//3)*3 + (j//3)].add(val)
        return True
