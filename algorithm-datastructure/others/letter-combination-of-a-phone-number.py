# answer contains O(3^n) elements(except for 9s)
# naive solution
# 

class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        mapper = {
            "2": ["a", "b", "c"],
            "3": ["d", "e", "f"],
            "4": ["g", "h", "i"],
            "5": ["j", "k", "l"],
            "6": ["m", "n", "o"],
            "7": ["p", "q", "r", "s"],
            "8": ["t", "u", "v"],
            "9": ["w", "x", "y", "z"]
        }
        ans = [""]
        for digit in digits:
            tmp = []
            for char in mapper[digit]:
                tmp += [item + char for item in ans]
            ans = tmp
        return ans