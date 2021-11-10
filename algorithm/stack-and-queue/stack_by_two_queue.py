class MyStack:

    def __init__(self):
        self.queue1 = []
        self.queue2 = []

    def push(self, x: int) -> None:
        if len(self.queue1) == 0:
            return self.queue2.append(x)
        else:
            return self.queue1.append(x)

    def pop(self) -> int:
        if len(self.queue1) == 0:
            for i in range(len(self.queue2) - 1):
                self.queue1.append(self.queue2.pop(0))
            return self.queue2.pop(0)
        else:
            for i in range(len(self.queue1) - 1):
                self.queue2.append(self.queue1.pop(0))
            return self.queue1.pop(0)

    def top(self) -> int:
        if len(self.queue1) == 0:
            for i in range(len(self.queue2) - 1):
                self.queue1.append(self.queue2.pop(0))
            el = self.queue2.pop(0)
            self.queue1.append(el)
            return el
        else:
            for i in range(len(self.queue1) - 1):
                self.queue2.append(self.queue1.pop(0))
            el = self.queue1.pop(0)
            self.queue2.append(el)
            return el

    def empty(self) -> bool:
        return len(self.queue1) == 0 and len(self.queue2) == 0
        


# Your MyStack object will be instantiated and called as such:
# obj = MyStack()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()
