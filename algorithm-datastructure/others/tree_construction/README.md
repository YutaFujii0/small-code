# Problem
https://codeforces.com/contest/675/problem/D

# Run

```bash
cat sample_input | cargo run
```

# Psuedocode

root node is fixed as the first element

for element in elements

- if head.val > element
  - check if assignment
  - head = head.right
- else (less than or equals to)
  - check if assignment
  - head = head.left

check if assignment
if head is null, asign node that holds value of element

Complexity:
- time complexity: depends on input, but in the worst case (linked list) it will n * O(n)
  if the input represents a balanced tree: it will be n * O(logn)
- space complextiy: O(n) for vector, deque, O(1) for iteration 