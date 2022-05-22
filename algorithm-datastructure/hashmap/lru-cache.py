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

class ListNode:
    def __init__(self, k = None, v = None):
        self.key = k
        self.val = v
        self.next = None
        self.prev = None

        

class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.head = ListNode()
        self.tail = self.head
        # key = ListNode
        self.cache = {}

    def get(self, key: int) -> int:
        if key in self.cache:
            # move reference order
            node = self.cache[key]
            if node != self.tail:
                node.prev.next = node.next
                if node.next:
                    node.next.prev = node.prev
                self.tail.next = node
                node.prev = self.tail
                self.tail = self.tail.next

            return self.cache[key].val
        else:
            return -1

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            self.cache[key].val = value
            # move reference order
            node = self.cache[key]
            if node == self.tail:
                return
            node.prev.next = node.next
            if node.next:
                node.next.prev = node.prev
            self.tail.next = node
            node.prev = self.tail
            self.tail = self.tail.next
        else:
            if len(self.cache) == self.capacity:
                self.cache.pop(self.head.next.key, None)
                self.head.next = self.head.next.next
                if self.head.next:
                    self.head.next.prev = self.head
                else:
                    self.tail = self.head
            new = ListNode(key, value)
            self.cache[key] = new
            # make referencing order
            self.tail.next = new
            new.prev = self.tail
            self.tail = self.tail.next


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)