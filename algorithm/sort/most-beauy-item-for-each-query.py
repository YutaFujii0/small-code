# WIP
# given price i, return max beauty of those have price LESS THAN OR EQUAL TO i
# to naively solve, even collect items of price <= i need to full scan O(n)
# in total, O(items * queries) needed, which is not good
# sort price hash
# for each query:
#   binary search the nearest price that is less than or equal to query

import math

class Solution:
    def maximumBeauty(self, items, queries):
        price_index = {}
        for item in items:
            if item[0] in price_index:
                # price_index[item[0]].append(item[1])
                price_index[item[0]] = max([item[1], price_index[item[0]]])
            else:
                price_index[item[0]] = item[1]

        self.index = self.merge_sort(list(price_index.items()))
        self.index = self.merge_sort([(3, 5),(5, 6),(2, 4),(1, 2)])

        return self.index
        # return [self.max_beauty_less_than_price(q) for q in queries]

    def merge_sort(self, items):
        if len(items) < 2:
            return items
        half = math.floor(len(items) / 2)
        l = items[0:half]
        r = items[half:]
        return self.merge(self.merge_sort(l), self.merge_sort(r))

    def merge(self, l, r):
        ret = []
        if len(l) > 0 and len(r) > 0:
            if l[0][0] > r[0][0]:
                ret.append(r.pop(0))
            else:
                ret.append(l.pop(0))
        if len(l) > 0:
            ret += l
        elif len(r) > 0:
            ret += r
        return ret

    # def max_beauty_less_than_price(self, q):


print(Solution().maximumBeauty([[1,2],[3,2],[2,4],[5,6],[3,5]], [1,2]))
