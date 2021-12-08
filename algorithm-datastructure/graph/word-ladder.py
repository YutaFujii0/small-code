from typing import List
# basic concept: wordList is a graph
# if wordList is formed into graph(vertix, edge), can I solve this?
# dijkstra's shortest algorithm
# from begin word to end word
# (in the loop, end word is come, done): 

# n: vertices, m: edges
# creating a graph:
# dijkstra's setup: heap O(n)
# find edge from begin node: ???
# dijkstra's loop: m * O(logn)

class Graph:
    def __init__(self, wordList: List[str]):
        length = len(wordList[0])
        self.nodes = []
        for word in wordList:
            self.nodes.append((word, []))

        def merge_sort(words):
            if len(words) == 1:
                return words
            split = len(words) // 2
            return merge(merge_sort(words[:split]), merge_sort(words[split:]))

        def merge(left, right):
            merged = []
            while left and right:
                if left[0][1] > right[0][1]:
                    merged.append(right.pop(0))
                else:
                    merged.append(left.pop(0))
            if left:
                merged += left
            if right:
                merged += right
            return merged

        dic1 = [(node, node[0][:length-1]) for node in self.nodes]
        print(merge_sort(dic1))



class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        Graph(wordList)
