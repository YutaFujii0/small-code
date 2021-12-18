# Programming Assignment

The goal of this problem is to implement the "Median Maintenance" algorithm.
The text file contains a list of the integers from 1 to 10000 in unsorted order; you should treat this as a stream of numbers, arriving one by one.
if k is odd, then m_k is ((k+1)/2)th smallest number among x_1,...,x_k;
if k is even, then m_k is the (k/2)th smallest number among x_1,...,x_k.
In the box below you should type the sum of these 10000 medians, modulo 10000.

OPTIONAL EXERCISE: Compare the performance achieved by heap-based and search-tree-based implementations of the algorithm.


# Psuedocode

Recap: the definition of median here is
if k is odd, then m_k is ((k+1)/2)th smallest number among x_1,...,x_k.
if k is even, then m_k is the (k/2)th smallest number among x_1,...,x_k.

- Two heaps (S: smaller half of values are stored, L: larger half of values are stored resp)
- On addition, if S[0] < val
  - insert val to L
- else
  - insert val to S
- until size(S) - size(L) is 0 or 1
  - rebalance(poll and insert)
- return head of S

# Original Problem

https://drive.google.com/file/d/1Gs_TqLPjdTQQQzP9zJPccAVghP-3iwEI/view?usp=sharing
