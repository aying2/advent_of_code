import re

class Grid3D:
    def __init__(self, filename:str):
        self.arr = self.parse(filename)

    def parse(self, filename:str) -> list:

        with open(filename) as file:
            exp = re.compile(r"(\d+),(\d+),(\d+)")
            
            max_dims = [0, 0, 0]
            for line in file:
                m = exp.match(line)
                if m:
                    g = m.groups()
                    max_dims[0] = max(max_dims[0], int(g[0]))
                    max_dims[1] = max(max_dims[1], int(g[1]))
                    max_dims[2] = max(max_dims[2], int(g[2]))
                else:
                    raise Exception("Invalid file")

            ret = self.build_empty(max_dims)

            file.seek(0)
            for line in file:
                m = exp.match(line)
                if m:
                    g = m.groups()
                    self.place(ret, int(g[0]), int(g[1]), int(g[2]))
                else:
                    raise Exception("Invalid file")
            
            return ret
    
    def build_empty(self, dims: list) -> list:
        return [[[0 for z in range(dims[2]+1)] for y in range(dims[1]+1)] for x in range(dims[0]+1)]


    def place(self, arr: list, x:int, y:int, z:int) -> None:
        if arr[x][y][z] == 0:
            arr[x][y][z] = 1
        else:
            breakpoint()
            raise Exception("duplicate insert")

    def count_faces(self):
        total = 0
        for i in range(len(self.arr)):
            for j in range(len(self.arr[0])):
                for k in range(len(self.arr[0][0])):
                    if self.arr[i][j][k] == 1:
                        total += self.analyze_cube(i, j, k)

        return total

    def count_faces(self):
        total = 0
        for i in range(len(self.arr)):
            for j in range(len(self.arr[0])):
                for k in range(len(self.arr[0][0])):
                    if self.arr[i][j][k] == 1:
                        total += self.analyze_cube(i, j, k)
                    else:
                        total -= self.subtract_air_pocket(i, j, k)

        return total
    
    def analyze_cube(self, x:int, y:int, z:int):
        total = 0
        if x <= 0 or self.arr[x-1][y][z] == 0:
            total += 1

        if x >= len(self.arr) - 1 or self.arr[x+1][y][z] == 0:
            total += 1

        if y <= 0 or self.arr[x][y-1][z] == 0:
            total += 1

        if y >= len(self.arr[0]) - 1 or self.arr[x][y+1][z] == 0:
            total += 1

        if z <= 0 or self.arr[x][y][z-1] == 0:
            total += 1

        if z >= len(self.arr[0][0]) - 1 or self.arr[x][y][z+1] == 0:
            total += 1

        return total
            

    def __str__(self) -> str:
        return self.arr.__str__()
            


if __name__ == "__main__":
    grid = Grid3D("input18.txt")

    print(grid)
    print(grid.count_faces())