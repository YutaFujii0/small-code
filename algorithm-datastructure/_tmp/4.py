class TextEditor:

    def __init__(self):
        self.text = ""
        self.textLen = 0
        self.cur = 0

    def addText(self, text: str) -> None:
        self.text = self.text[0:self.cur] + text + self.text[self.cur:]
        self.cur += len(text)
        self.textLen += len(text)

    def deleteText(self, k: int) -> int:
        delta = min(self.cur, k)
        self.text = self.text[0:self.cur - delta] + self.text[self.cur:]
        self.cur -= delta
        self.textLen -= delta
        return delta

    def cursorLeft(self, k: int) -> str:
        delta = min(self.cur, k)
        self.cur -= delta
        start = max(0, self.cur - 10)
        return self.text[start:self.cur]

    def cursorRight(self, k: int) -> str:
        self.cur = min(self.cur + k, self.textLen)
        start = max(0, self.cur - 10)
        return self.text[start:self.cur]


# Your TextEditor object will be instantiated and called as such:
obj = TextEditor()
param_1 = obj.addText("leetcode")
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")
param_2 = obj.deleteText(4)
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")
param_3 = obj.addText("practice")
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")
param_4 = obj.cursorRight(3)
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")
param_5 = obj.cursorLeft(8)
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")
param_6 = obj.deleteText(10)
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")
param_7 = obj.cursorLeft(2)
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")
param_8 = obj.cursorRight(6)
print(f"{obj.text}, cur at {obj.cur}, len is {obj.textLen}")

print("-------------")
print(param_1)
print(param_2)
print(param_3)
print(param_4)
print(param_5)
print(param_6)
print(param_7)
print(param_8)