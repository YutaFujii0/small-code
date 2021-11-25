from typing import List
# Heap structure
# to keep track of kth LARGEST number among n,
# set MIN heap that has only k element(and implement it with array)
# every insertion, add element to heap and heapify
# to return kth largetst element, read first element of heap
# (not polling)
# __init__()
# - store all element in heap(List[int])
# - set size
# - until size == k:
#   - heap.poll()
#   - size --
# add()
# - add element to heap
# - heapify
# - poll()
# - read first element

class KthLargest:

    def __init__(self, k: int, nums: List[int]):
        # self.nums = nums
        self.k = k
        self.heap = []
        self.size = 0
        for num in nums:
            self.add(num)

    def add(self, val: int) -> int:
        if len(self.heap) == 0 or val > self.heap[0] or self.size < self.k:
            self.heap.append(val)
            self.size += 1
            self.heapify_up()
            while self.size > self.k:
                self.heap_poll()
        return self.heap[0]

    def heapify_up(self):
        child_index = self.size - 1
        parent_index = (child_index - 1) // 2
        while child_index != 0 and self.heap[parent_index] > self.heap[child_index]:
            tmp = self.heap[parent_index]
            self.heap[parent_index] = self.heap[child_index]
            self.heap[child_index] = tmp
            child_index = parent_index
            parent_index = (child_index - 1) // 2

    def heapify_down(self):
        parent_index = 0
        left_child_index = parent_index * 2 + 1
        while left_child_index < self.size:
            smallest_child_index = left_child_index
            if left_child_index + 1 < self.size:
                if self.heap[left_child_index] > self.heap[left_child_index + 1]:
                    smallest_child_index += 1

            if self.heap[parent_index] > self.heap[smallest_child_index]:
                tmp = self.heap[parent_index]
                self.heap[parent_index] = self.heap[smallest_child_index]
                self.heap[smallest_child_index] = tmp
            parent_index = smallest_child_index
            left_child_index = parent_index * 2 + 1

    def heap_poll(self) -> Optional[int]:
        if self.size == 0:
            return None
        if self.size == 1:
            return self.heap.pop()

        head = self.heap[0]
        self.heap[0] = self.heap[-1]
        self.heap.pop(-1)
        self.size -= 1
        self.heapify_down()

        return head
