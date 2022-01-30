from typing import List
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

class Solution:
    def groupStrings(self, words: List[str]) -> List[int]:
        groupDict = {}
        inverseDict = {}
        groupId = 0
        for word in words:
            n = len(word)
            if not any([word[2:i] + word[i+1:] in groupDict for i in range(n)]):
                groupId += 1
            if groupId in inverseDict.keys():
                inverseDict[groupId] += 1
            else:
                inverseDict[groupId] = 1
            for i in range(n):
                groupDict[word[:i] + word[i+1:]] = groupId
            groupDict[word] = groupId
        return [groupId, max(inverseDict.values())]
