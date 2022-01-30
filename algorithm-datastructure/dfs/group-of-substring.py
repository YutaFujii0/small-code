from typing import List
from collections import defaultdict
from collections import Counter
# idea hash
# Psuedocode
# gDictionary = {}
# inverseDictionary = {}
# groupId = 1
# for word in words
#   for i in len(word)
#     - lookup word[:i] + word[i+1:]
#   if any hits, get group id, else group_id +=1 
#   insert inverse disctionary
#     for i in len(word)
#       - insert dict key word[:i] + word[i+1:] value group id
# return groupId and max of values

import string


class Solution:
    def groupStrings(self, words: List[str]) -> List[int]:
        # each word contains at the maximum 26 letters, thus we need to store at least 26 for each char
        int2str = { int(c,32): c for c in (string.digits+string.ascii_lowercase)[:32] }
        G = defaultdict(set)
        M = defaultdict()
        
        def frequencyMask(s: str) -> str:
            mask = [0] * 26
            for char in s:
                mask[ord(char) - ord("a")] += 1
            return "".join([int2str[v] for v in mask])

        # store 26 chars of alphabet frequency for each word and its masks
        for index, word in enumerate(words):
            for i in range(len(word)):
                M[frequencyMask(word[:i]+word[i+1:])] = index
            M[frequencyMask(word)] = index

        # creating graph
        for x, word in enumerate(words):
            if not G[x]: G[x] = set()
            n = len(word)
            for i in range(n+1):
                y = M[frequencyMask(word[:i]+word[i+1:])]
                if y != x:
                    G[x].add(y)
                    G[y].add(x)

        # count connected components with DFS
        explored = set()
        cc = 0
        globalMax = 0

        def dfs(node):
            if node in explored:
                return 0

            explored.add(node)
            return sum([dfs(nextNode) for nextNode in G[node]]) + 1

        for node in G.keys():
            if not node in explored:
                localMax = dfs(node)
                if globalMax < localMax: globalMax = localMax
                cc += 1

        return cc, globalMax
        # return [cc, max([len(nodes) for nodes in G.values()])+1]

# class Solution:
#     def groupStrings(self, w):
#         d = {word: i for i, word in enumerate(w)}
#         mask = lambda x: sum(1<<(ord(i) - ord("a")) for i in x)
#         M = {mask(word): i for i, word in enumerate(w)}
#         print(M)

#         G = defaultdict(list)
#         for t, word in enumerate(w):
#             vals = [ord(i) - ord("a") for i in word]
#             mask = sum(1<<i for i in vals)
#             for i in vals:
#                 if mask - (1<<i) not in M: continue
#                 idx2 = M[mask - (1<<i)]
#                 G[t] += [idx2]
#                 G[idx2] += [t]

#         masks = defaultdict(list)
#         for idx, word in enumerate(w):
#             vals = [ord(i) - ord("a") for i in word]
#             mask = sum(1<<i for i in vals)
#             for i in vals:
#                 mask2 = mask - (1<<i) + (1<<26)
#                 masks[mask2].append(idx)
        
#         for x in masks:
#             for y, z in zip(masks[x], masks[x][1:]):
#                 G[y] += [z]
#                 G[z] += [y]

#         V, idx = set(), 0
#         comps = Counter()

#         def dfs(node, idx):
#             V.add(node)
#             comps[idx] += 1
#             for child in G[node]:
#                 if child not in V:
#                     dfs(child, idx)

#         for node in range(len(w)):
#             if node not in V:
#                 dfs(node, idx)
#                 idx += 1

#         return len(comps), max(comps.values())

print(Solution().groupStrings(["a","b","ab","cde"]))
print(Solution().groupStrings(["a","ab","abc"]))
print(Solution().groupStrings(["a","bb","dbc"]))
print(Solution().groupStrings(["aa","b"]))
print(Solution().groupStrings(["aa","bb"]))
print(Solution().groupStrings(["ghnv","uip","tenv","hvepx","e","ktc","byjdt","ulm","ea","cae"]))
print(Solution().groupStrings(["ghnv","uip","tenv","hvepx","e","ktc","byjdt","ulm","cae","ea"]))
print(Solution().groupStrings(["web","a","te","hsx","v","k","a","roh"]))
print(Solution().groupStrings(["ep","x","e","hj","sxru","vsbt","akwdp","q","vyle","lip"]))
