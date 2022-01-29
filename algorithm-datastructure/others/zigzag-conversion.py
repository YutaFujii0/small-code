# idea: stack
# set numRows of stacks(stack0,stack1,stack2,...)
# for each char in s, push stack0, 1, 2,...,2,1,0...
# join chars in each stack, then join them
# Complexity:
#  - time complexity: O(1) * n for pushing, O(n) + O(numRow) for joining
#  - space complexity: O(n) for stacks

class Solution:
    def convert(self, s: str, numRows: int) -> str:
        if numRows == 1:
            return s
        stacks = [[] for _ in range(numRows)]
        i = 0
        
        for char in s:
            index = abs(abs((i+numRows-1) % (2 * (numRows-1))) - numRows + 1)
            stacks[index].append(char)
            i += 1

        return "".join(["".join(stack) for stack in stacks])
