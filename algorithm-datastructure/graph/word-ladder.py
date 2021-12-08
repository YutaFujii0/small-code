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
            tmp_dic = [(node, node[0][:length-i-1]+node[0][length-i:]) for node in self.nodes]
            tmp_hash = {}
            for item in tmp_dic:
                if item[1] in tmp_hash:
                    tmp_hash[item[1]].append(item[0])
                else:
                    tmp_hash[item[1]] = [item[0]]
            for key in tmp_hash:
                for node in tmp_hash[key]:
                    for connected_node in tmp_hash[key]:
                        if node != connected_node:
                            node[1].append(connected_node)

    def traverse(self, endWord: str):
        explored = {}
        queue = [[self.nodes[0]]]
        hops = 1
        while queue and queue[0]:
            nodes = queue.pop(0)
            next_iter = []
            for node in nodes:
                if node[0] == endWord:
                    return hops
                if node[0] in explored:
                    continue
                explored[node[0]] = hops
                next_iter += node[1]
                
            queue.append(next_iter)
            hops += 1
        return 0

class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        graph = Graph([beginWord] + wordList)
        return graph.traverse(endWord)
