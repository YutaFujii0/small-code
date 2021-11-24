from typing import List
# keep nums fully sorted
# - merge sort at initialization?
# - sort(how?) every addition?
#  - if data is array or linked list, insertion need O(n) time

# heap is only top max/min value ensured, not kth sorted value

class KthLargest:

    def __init__(self, k: int, nums: List[int]):
        self.sorted_nums = self.merge_sort(nums)
        self.k = k

    def add(self, val: int) -> int:
        i = 0
        while i < len(self.sorted_nums) and self.sorted_nums[i] >= val:
            i += 1
        self.sorted_nums = self.sorted_nums[:i] + [val] + self.sorted_nums[i:]
        return self.sorted_nums[self.k - 1]

    def merge_sort(self, nums: List[int]) -> List[int]:
        if len(nums) < 2:
            return nums

        left = nums[:len(nums) // 2]
        right = nums[len(nums) // 2:]
        return self.merge(self.merge_sort(left), self.merge_sort(right))

    def merge(self, left: List[int], right: List[int]) -> List[int]:
        merged_list = []
        while len(left) != 0 and len(right) != 0:
            if left[0] < right[0]:
                merged_list.append(right.pop(0))
            else:
                merged_list.append(left.pop(0))
        if len(left) > 0:
            merged_list += left
        if len(right) > 0:
            merged_list += right
        return merged_list


# Your KthLargest object will be instantiated and called as such:
k = 1
nums = []
nums = [1,45,3,6]
obj = KthLargest(k, nums)
print(obj.sorted_nums)
param_1 = obj.add(9)
print(obj.sorted_nums)
print(param_1)
param_1 = obj.add(9)
print(obj.sorted_nums)
print(param_1)
