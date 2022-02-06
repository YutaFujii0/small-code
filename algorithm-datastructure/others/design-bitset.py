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
