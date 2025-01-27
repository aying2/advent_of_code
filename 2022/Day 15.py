import re
import copy

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

class SensorArray(SparseArray):
    def __init__(self, filename: str):
        super().__init__()
        self.parse(filename)

    def parse(self, filename: str) -> None:
        with open(filename) as file:
            exp = re.compile(r"(-?\d+).+?(-?\d+).+?(-?\d+).+?(-?\d+)")

            for i, line in enumerate(file):
                match = exp.findall(line)
                if len(match) > 0:
                    for tup in match:
                        self.put((int(tup[0]), int(tup[1])), (int(tup[2]), int(tup[3])))    
    
    def taxi_dist(self, pos1: tuple, pos2: tuple) -> int:
        return abs(pos1[0] - pos2[0]) + abs(pos1[1] - pos2[1])

    # very slow and naive
    def analyze_row(self, y = 10) -> int:
        min_x = 0
        max_x = 0
        for sensor_pos, beacon_pos in self.array.items():
            radius = self.taxi_dist(sensor_pos, beacon_pos)
            if sensor_pos[1] - radius < min_x:
                min_x = sensor_pos[1] - radius
            if sensor_pos[1] + radius > max_x:
                max_x = sensor_pos[1] + radius

        # print(min_x, max_x)

        row = ["." for i in range(min_x, max_x + 1)]

        # print(row)
        # print(len(row))

        for i, x in enumerate(range(min_x, max_x + 1)):
            for sensor_pos, beacon_pos in self.array.items():
                radius = self.taxi_dist(sensor_pos, beacon_pos)
                row_pos = (x, y)
                row_dist = self.taxi_dist(sensor_pos, row_pos)

                if row_dist <= radius and row_pos not in self.array.values() and row_pos not in self.array.keys():
                    row[i] = "#"
        # print(row)
        return row.count("#")

    # also slow; would be better to scan and jump diagonally
    def find_pos(self, bound = 4000000) -> tuple|None:
 
        y = 0
        while y < bound + 1:
            print(y)
            x = 0
            while x < bound + 1:
                row_pos = (x, y)
                covered = False
                for sensor_pos, beacon_pos in self.array.items():
                    radius = self.taxi_dist(sensor_pos, beacon_pos)
                    
                    row_dist = self.taxi_dist(sensor_pos, row_pos)

                    if row_dist <= radius:
                        covered = True
                        
                        # we know for this row where the end of the dead region is
                        # because it is symmetric about the sensor
                        #  First jump the x displacement between current pos and sensor
                        # Then, we know the radius and displacement in y between the sensor and the current pos
                        # so we can solve for the x leg
                        x_jump = sensor_pos[0] - row_pos[0] + radius - abs(y - sensor_pos[1])
                        x += x_jump
                        break
                x += 1
                if not covered:
                    return row_pos
            y += 1
                
                
            
        



if __name__ == "__main__":

    arr = SensorArray("input15.txt")

    print(arr)

    # print(arr.analyze_row(y=2000000))

    found = arr.find_pos()
    print(found)

    if found:
        print(found[0]*4000000 + found[1])

