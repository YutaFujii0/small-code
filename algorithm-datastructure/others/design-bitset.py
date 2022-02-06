class Bitset:

    def __init__(self, size: int):
        self.bits = [False] * size
        

    def fix(self, idx: int) -> None:
        self.bits[idx] = True
        

    def unfix(self, idx: int) -> None:
        self.bits[idx] = False

    def flip(self) -> None:
        self.bits = [not i for i in self.bits]

        # for i in self.bits:
        #     i = not i

    def all(self) -> bool:
        return all(self.bits)

    def one(self) -> bool:
        return any(self.bits)

    def count(self) -> int:
        return sum([1 for i in self.bits if i])

    def toString(self) -> str:
        return "".join(['1' if i else '0' for i in self.bits])


class BitsetImproved:
    def __init__(self, size: int):
        self.bits = 1 << size
        self.cnt = 0
        self.size = size
        
    def fix(self, idx: int) -> None:
        idx = self.size - idx - 1
        if self.bits & (1 << idx) == 0: self.cnt += 1
        self.bits = self.bits | (1 << idx)
        
    def unfix(self, idx: int) -> None:
        idx = self.size - idx - 1
        if self.bits & (1 << idx) > 0: self.cnt -= 1
        self.bits &= (((1 << self.size+1) - 1) ^ (1 << idx))

    def flip(self) -> None:
        self.bits ^= ((1 << self.size) - 1)
        self.cnt = self.size - self.cnt
        # for i in self.bits:
        #     i = not i

    def all(self) -> bool:
        return self.bits == ((1 << self.size + 1) - 1)

    def one(self) -> bool:
        return self.bits > (1 << self.size)

    def count(self) -> int:
        return self.cnt

    def toString(self) -> str:
        return bin(self.bits)[3:]



bs = Bitset(5)# // bitset = "00000".
print(bs.flip())    # // the value of each bit is flipped, so bitset = "10101". 
print(bs.toString())# // return "01010", which is the composition of bitset.
print(bs.toString())# // return "01010", which is the composition of bitset.
print(bs.count())   # // return 2, as there are 2 bits with value 1.
print(bs.flip())    # // the value of each bit is flipped, so bitset = "10101". 
print(bs.fix(3))    # // the value at idx = 1 is updated to 1, so bitset = "01010". 
print(bs.fix(1))    # // the value at idx = 1 is updated to 1, so bitset = "01010". 
print(bs.toString())# // return "01010", which is the composition of bitset.
print(bs.count())   # // return 2, as there are 2 bits with value 1.
print(bs.flip())    # // the value of each bit is flipped, so bitset = "10101". 
print(bs.unfix(0))    # // the value at idx = 3 is updated to 1, so bitset = "00010".
print(bs.toString())# // return "01010", which is the composition of bitset.
print(bs.one())    # // the value at idx = 3 is updated to 1, so bitset = "00010".
print(bs.count())   # // return 2, as there are 2 bits with value 1.
print(bs.toString())# // return "01010", which is the composition of bitset.
