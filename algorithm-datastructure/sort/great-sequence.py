# codeforce 773

import sys
from collections import Counter

class Solution:
    def call(self, input):
        problems = self.parse(input)
        # print(problems)
        for problem in problems:
            self.solve(problem)
    
    def parse(self, input):
        raw = [text.split(" ") for text in input]
        # omit first line
        raw.pop(0)
        problems = []
        for i in range(len(raw)//2):
            problems.append({ "n": int(raw[2*i][0]), "x": int(raw[2*i][1]), "nums": [int(a) for a in raw[2*i+1]] })
        return problems
    
    # general idea: greedy algorithm. pick least connected edge
    # ex)
    # 1 - 10 - 100 - 1000 - 10000
    #   - 10 -
    # then pick 1000 - 10000 because this is least connected edge
    # But this need to maintain least edges over time (need nlogn)
    # But, this time we can reduce another way
    # say, nums are like above example, represent this like
    # [1, 2, 1, 1, 1]
    # and from left to right, reduce nodes next to each other as much as possible
    # [1, 2, 1, 1, 1]
    # [0, 1, 1, 1, 1] (first iteration)
    # [0, 0, 0, 1, 1] (2nd iteration)
    # [0, 0, 0, 1, 1] (3rd iteration)
    # [0, 0, 0, 0, 0] (4th iteration) --> this tree can be separated completely

    # another example
    # 1 - 10 - 100 - 1000 - 10000
    # 1 -    - 100 - 1000
    # 1 -    - 100
    # representation: [3, 1, 3, 2, 1]
    # [2, 0, 3, 2, 1] (1st iteration)
    # [2, 0, 3, 2, 1] (2nd iteration)
    # [2, 0, 1, 0, 1] (3rd iteration)
    # [2, 0, 1, 0, 1] (4th iteration) -> need another 4 integers to make pairs

    # Psuedocode(time complexity)
    # count frequencies: O(n)
    # sort int: O(nlogn)
    # cluster trees, meaning make group of *x integers: O(n)
    # conduct process above: O(n)
    # sum of remaining int is the answer
    # Complexity
    # - time complexity: O(nlogn)
    # - space complexity: O(n) for clusters, frequency counter
    def solve(self, problem):
        # greedy algorithm
        # count
        freq = dict(Counter(problem["nums"]))
        cluster = []

        explored = set()
        nums = list(freq)
        nums.sort()
        for num in nums:
            if num in explored:
                continue
            explored.add(num)
            group = [freq[num]]
            while num * problem["x"] in freq.keys():
                num *= problem["x"]
                explored.add(num)
                group.append(freq[num])
            cluster.append(group)

        remaining = 0
        for group in cluster:
            for i in range(len(group)-1):
                pairs = min(group[i], group[i+1])
                group[i] -= pairs
                group[i+1] -= pairs
            remaining += sum(group)
        print(remaining)
        return remaining


# input = """4
# 4 4
# 1 16 4 4
# 6 2
# 1 2 2 2 4 7
# 5 3
# 5 2 3 5 15
# 9 10
# 10 10 10 20 1 100 200 2000 3"""

input = sys.stdin.readlines()
Solution().call(input)