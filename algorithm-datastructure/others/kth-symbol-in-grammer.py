# idea: trace back ancestors of kth element
# 1) how to determine the path
#   if k = 0(mod2), then that element is derived from RIGHT node of the parent
#   if k = 1(mod2), it's left node of the parent
# This way, you can see the path to kth element
#   ex) root - left - right - right - right (n=5 k=8)
# 
# 2) here's how digit is determined
#   parent\child | left | right 
#             0  |  0   |  1
#             1  |  1   |  0
# -> let left=0 and right=1, then this is bitwise XOR operation
# 
# From these observations, you can calculate answer as follows
# Psuedocode
# - path_stack = []
# - while k > 0
#   - derive_digit = 1 - (k % 2)
#   - push derive_digit to stack
#   - k // 2
# - current_digit = 0
# - while stack is not empty
#   - derive_digit = stack.pop()
#   - current_digit = current_digit XOR derive_digit
# - return current_digit
# Complexity:
# - time complexity: 2 * O(logn)
# - space complexity: O(logn) for stack + O(1) for current_digit

class Solution:
    def kthGrammar(self, n: int, k: int) -> int:
        stack = []
        while n > 1:
            stack.append(1 - (k % 2))
            k = (k + 1) // 2
            n -= 1
        current_digit = 0
        while stack:
            current_digit = current_digit ^ stack.pop()
        return current_digit
