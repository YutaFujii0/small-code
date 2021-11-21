class Robot:

    def __init__(self, width: int, height: int):
        # self.directions = ["East", "North", "West", "South"]
        self.width = width
        self.height = height
        self.round = (self.width + self.height) * 2 - 4
        self.position = [0, 0]
        self.direction = "East"
        self.d_position = 0

    def move(self, num: int) -> None:
        rem = (num + self.d_position) % (self.round)
        self.d_position = rem
        self.position = [0,0]
        if rem == 0:
            self.direction = "South"
            return
        if rem >= self.width:
            self.position[0] = self.width - 1
            rem -= (self.width - 1)
        else:
            self.position[0] = rem
            self.direction = "East"
            return
        if rem >= self.height:
            self.position[1] = self.height - 1
            rem -= (self.height - 1)
        else:
            self.position[1] = rem
            self.direction = "North"
            return
        if rem >= self.width:
            self.position[0] = 0
            rem -= (self.width - 1)
        else:
            self.position[0] -= rem
            self.direction = "West"
            return
        if rem >= self.height:
            self.position[1] = 0
            rem -= (self.height - 1)
        else:
            self.position[1] -= rem
            self.direction = "South"
            return
        
        
        
        # if num <= 0:
        #     return
        # move_to = [self.position[0] + self.direction[0], self.position[1] + self.direction[1]]
        # if move_to[0] < self.width and move_to[1] < self.height and move_to[0] >= 0 and move_to[1] >= 0:
        #     self.position = move_to
        #     self.move(num - 1)
        # else:
        #     self.rotate()
        #     self.move(num)

    def getPos(self) -> List[int]:
        return self.position
        
    def getDir(self) -> str:
        return self.direction


# Your Robot object will be instantiated and called as such:
# obj = Robot(width, height)
# obj.move(num)
# param_2 = obj.getPos()
# param_3 = obj.getDir()
