import re

class SparseArray:
    def __init__(self):
        self.array = {}
    
    def put(self, pos: tuple, val) -> None:
        self.array[pos] = val

    def get(self, pos: tuple):
        return self.array[pos]

    def has(self, pos: tuple) -> bool:
        try:
            self.array[pos]
            return True
        except KeyError:
            return False


    def __str__(self):
        return self.array.__str__()

class SandSim(SparseArray):
    def __init__(self, filename: str):
        super().__init__()
        self.highest = 0
        self.parse(filename)

    def set_rock(self, pos: tuple) -> None:
        super().put(pos, 1)
        if pos[1] > self.highest:
            self.highest = pos[1] 


    def set_sand(self, pos: tuple) -> None:
        super().put(pos, 2)

    def set_rock_path(self, start: tuple, end: tuple) -> None:
        if start[0] == end[0]:
            minpos = min(start[1], end[1])
            maxpos = max(start[1], end[1])
            for i in range(minpos, maxpos + 1):
                self.set_rock((start[0], i))
        elif start[1] == end[1]:
            minpos = min(start[0], end[0])
            maxpos = max(start[0], end[0])
            for j in range(minpos, maxpos + 1):
                self.set_rock((j, start[1]))
        else:
            raise Exception("Invalid rock path")

    def set_rock_paths(self, poses: list) -> None:
        for i in range(len(poses) - 1):
            self.set_rock_path(poses[i], poses[i + 1])

    def parse(self, filename: str):
        with open(filename) as file:
            exp = re.compile(r"(\d+),(\d+)")

            for i, line in enumerate(file):
                match = exp.findall(line)
                if len(match) > 0:
                    to_int = [(int(elem[0]), int(elem[1])) for elem in match]
                    self.set_rock_paths(to_int)

    def simulate_p1(self, source: tuple = (500, 0)) -> int:
        count = 0
        while True:
            pos = self.fall(source)
            if pos is None:
                break
            self.set_sand(pos)
            count += 1

        return count
    
    def simulate_p2(self, source: tuple = (500, 0)) -> int:
        count = 0
        while True:
            pos = self.fall_stop(source)
            self.set_sand(pos)
            count += 1
            if pos == source:
                break

        return count
    
    def fall(self, pos: tuple) -> tuple|None:
        down = (pos[0], pos[1] + 1)
        down_left = (pos[0] - 1, pos[1] + 1)
        down_right = (pos[0] + 1, pos[1] + 1)
        # i.e. falling to abyss
        if pos[1] > self.highest:
            return None
        # straight down
        elif not self.has(down):
            return self.fall(down)
        # down left
        elif not self.has(down_left):
            return self.fall(down_left)
        # down right
        elif not self.has(down_right):
            return self.fall(down_right)
        else:
            return pos

    def fall_stop(self, pos: tuple) -> tuple:
        down = (pos[0], pos[1] + 1)
        down_left = (pos[0] - 1, pos[1] + 1)
        down_right = (pos[0] + 1, pos[1] + 1)
        
        if down[1] == self.highest + 2:
            return pos
        # straight down
        elif not self.has(down):
            return self.fall_stop(down)
        # down left
        elif not self.has(down_left):
            return self.fall_stop(down_left)
        # down right
        elif not self.has(down_right):
            return self.fall_stop(down_right)
        else:
            return pos

        

if __name__ == "__main__":

    sim = SandSim("input14.txt")

    print(sim)

    print(sim.simulate_p2())

    
