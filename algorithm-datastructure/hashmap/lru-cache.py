# LRU cache
# least recent used
# - queue (if implemented simply, it is least recent pushed)
# - need something when get is called(=used)
# constrains:
# - get/put must run in O(1) thus
# - what does "average" imply?
# - cannot use heap with put (needs O(logn) for push operation)
# - cannot use binary tree/list with get (needs O(n) for search operation)
# least recent used
# - deque with capacity of n
# put
#  if it exists
#   - update value in hash table
#  else
#   - pop if capacity == n
#   - push_front key into queue and insert key=value to hash table
#  - push front key into queue
# get
# - read hash table
# - push front referenced key to queue if it exists


class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.referencedDeque = deque()
        self.cache = {}

    def get(self, key: int) -> int:
        if key in self.cache:
            if len(self.referencedDeque) == self.capacity:
                self.referencedDeque.pop()
            self.referencedDeque.appendleft(key)
            return self.cache[key]
        else:
            return -1

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            self.cache = value
            if len(self.referencedDeque) == self.capacity:
                self.referencedDeque.pop()
            self.referencedDeque.appendleft(key)
        else:
            if len(self.cache) == 2:
                self.cache.pop(self.referencedDeque.pop(), None)
            self.cache[key] = value
            self.referencedDeque.appendleft(key)


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)