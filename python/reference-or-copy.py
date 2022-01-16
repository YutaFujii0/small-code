a = []
b = []

for i in range(10):
    b.append(i)
    a.append(b)
    b.pop()

print(a)

# -> [[], [], [], [], [], [], [], [], [], []]



a = []
b = []

for i in range(10):
    b.append(i)
    a.append(b[:])
    b.pop()

print(a)

# -> [[0], [1], [2], [3], [4], [5], [6], [7], [8], [9]]
