# codesignal
# count up digit occurances: using hash
# then, sort digits by frequency, naively O(n^2) or O(nlogn) by merge sort(or heap)
# Time complexity: O(2n) for counting, O(nlogn) for sorting
# Space complexity: O(1) for count hash, O(nlogn) for merge sort
def solution(a):
    # count up
    frequencies = {i: 0 for i in range(10)}
    for num in a:
        while num:
            frequencies[num % 10] += 1
            num //= 10

    # merge sort by frequency(TODO: use heap)
    def merge_sort(items):
        if len(items) < 2:
            return items
        half = len(items) // 2
        left = merge_sort(items[:half])
        right = merge_sort(items[half:])
        return merge(left, right)

    def merge(left, right):
        ans = []
        while left and right:
            # [0][1] points frequency of head digit
            # if frequency is the same, take smaller digit first
            if left[0][1] == right[0][1]:
                if left[0][0] > right[0][0]:
                    ans.append(right.pop(0))
                else:
                    ans.append(left.pop(0))
            elif left[0][1] < right[0][1]:
                ans.append(right.pop(0))
            else:
                ans.append(left.pop(0))
        if left:
            ans += left
        if right:
            ans += right
        return ans

    freqs = [(k,v) for k,v in frequencies.items()]
    freqs = merge_sort(freqs)

    # take most frequent
    ans = [freqs[0][0]]
    i = 0
    while i < 9 and freqs[i][1] == freqs[i+1][1]:
        ans.append(freqs[i+1][0])
        i += 1

    return ans
