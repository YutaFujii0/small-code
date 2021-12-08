from typing import List
# basic concept: wordList is a graph
# if wordList is formed into graph(vertix, edge), can I solve this?
# dijkstra's shortest algorithm
# or just BFS (since lenght of edge is all the same)
# from begin word to end word
# (in the loop, end word is come, done):

# n: vertices, m: edges
# Complexity
# - time complexity
#  - creating a graph: 
#  - BFS: O(m*n)

class Graph:
    def __init__(self, wordList: List[str]):
        length = len(wordList[0])
        self.nodes = []
        for word in wordList:
            self.nodes.append((word, []))

        for i in range(length):
            tmp_hash = {}
            # {'it': [<node>, <node>, <node>], 'og': [<node>, <node>, <node>], ... }
            for node in self.nodes:
                key = node[0][:length-i-1]+node[0][length-i:]
                tmp_hash.setdefault(key, []).append(node)
            for key, nodes in tmp_hash.items():
                for node in nodes:
                    for connected_node in nodes:
                        if node != connected_node:
                            node[1].append(connected_node)

    def traverse(self, endWord: str):
        explored = {}
        queue = [[self.nodes[0]]]
        layer = 1
        while queue and queue[0]:
            nodes = queue.pop(0)
            next_iter = []
            for node in nodes:
                if node[0] == endWord:
                    return layer
                if node[0] in explored:
                    continue
                explored[node[0]] = layer
                next_iter += node[1]

            queue.append(next_iter)
            layer += 1
        return 0

class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        graph = Graph([beginWord] + wordList)
        return graph.traverse(endWord)
