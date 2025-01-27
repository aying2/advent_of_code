import sys

def build_grid(filename):
    with open(filename) as ifile:
        grid = []
        for line in ifile:
            grid.append(line.strip())
        return grid


def find_trees(grid):
    count = 0
    for i, line in enumerate(grid):
        for j, n in enumerate(line):
            if i==0 or i==len(grid)-1 or j==0 or j==len(line)-1:
                count+=1
                
            else:
                clearL = True
                clearR = True
                clearT = True
                clearB = True
                for i2, line2 in enumerate(grid):
                    for j2, n2 in enumerate(line2):
                        if j2<j and i2==i and n2>=n:
                            clearL = False
                        if j2>j and i2==i and n2>=n:
                            clearR = False
                        if i2<i and j2==j and n2>=n:
                            clearT = False
                        if i2>i and j2==j and n2>=n:
                            clearB = False
                if clearL or clearR or clearT or clearB:
                    count+=1
    return count

def find_max_view_score(grid):
    max_score = 0
    for i, line in enumerate(grid):
        for j, n in enumerate(line):
            treesL = line[0:j]
            treesR = line[j+1:len(line)]
            treesT = []
            treesB = []
            for i2, line2 in enumerate(grid):
                if i2 < i:
                    treesT.append(line2[j])
                if i2 > i:
                    treesB.append(line2[j])
            treesL = treesL[::-1]
            treesT = treesT[::-1]
            scoreL, scoreR, scoreT, scoreB = 0, 0, 0, 0
            for n2 in treesL:
                scoreL+=1
                if n2>= n:
                    break
            for n2 in treesR:
                scoreR+=1
                if n2>= n:
                    break
            for n2 in treesT:
                scoreT+=1
                if n2>= n:
                    break
            for n2 in treesB:
                scoreB+=1
                if n2>= n:
                    break
            total_score = scoreL * scoreR * scoreT * scoreB
            if total_score>max_score:
                max_score = total_score
    return max_score

filename = sys.argv[1]
grid = build_grid(filename)

print(find_max_view_score(grid))

# print(find_trees(grid))