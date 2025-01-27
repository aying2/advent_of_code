import sys
import math
import re
import copy

def count_visited(filename):
    with open(filename) as ifile:
        headPos = [0,0]
        tailPos = [0,0]

        visitedPos = [tailPos.copy()]

        oldHeadPos = headPos.copy()
        for line in ifile:
            direction = line[0]
            distance = eval(re.findall('\d+', line)[0])

            for i in range(distance):
                if direction == 'L':
                    headPos[0] -= 1
                if direction == 'R':
                    headPos[0] += 1
                if direction == 'D':
                    headPos[1] -= 1
                if direction == 'U':
                    headPos[1] += 1
        
                diffH = headPos[0]-tailPos[0]
                diffV = headPos[1]-tailPos[1]

                difference = math.sqrt((diffH)**2 + (diffV)**2)
                # print(difference)
                if difference >= 2:
                    tailPos = oldHeadPos.copy()
                    visitedPos.append(tailPos.copy())
                
                oldHeadPos = headPos.copy()
            
    visitedPos = set([tuple(pos) for pos in visitedPos])
    
    return len(visitedPos)
                    
def count_visited_more_knots(filename, n = 10):
    with open(filename) as ifile:
        knotPoses = [[0,0] for i in range(n)]

        visitedPos = [knotPoses[n-1].copy()]

        for line in ifile:
            direction = line[0]
            distance = eval(re.findall('\d+', line)[0])

            for i in range(distance):
                # move head
                if direction == 'L':
                    knotPoses[0][0] -= 1
                if direction == 'R':
                    knotPoses[0][0] += 1
                if direction == 'D':
                    knotPoses[0][1] -= 1
                if direction == 'U':
                    knotPoses[0][1] += 1

                for i in range(n-1):
                    # update difference as each knot moves
                    difference = find_distance(knotPoses[i], knotPoses[i+1])
                    
                    if difference >= 2:
                        # find all possible positions the current knot can move to (within 1 space)
                        # first search for positions directly adjacent
                        found = False
                        for i2 in range(-1, 2):
                            for j2 in range(-1, 2):
                                newPos = [knotPoses[i+1][0]+i2, knotPoses[i+1][1]+j2]
                                newDiff = find_distance(knotPoses[i], newPos)
                                if  newDiff == 1 and not found:
                                    found = True
                                    knotPoses[i+1] = newPos.copy()
                        # kind of degenerate, but if no move was directly adjacent, the ahead knot
                        # was already diagonal and made a diagonal move, so find that move
                        if not found:
                            for i2 in range(-1, 2):
                                for j2 in range(-1, 2):
                                    newPos = [knotPoses[i+1][0]+i2, knotPoses[i+1][1]+j2]
                                    newDiff = find_distance(knotPoses[i], newPos)
                                    if  newDiff < 2 and not found:
                                        found = True
                                        knotPoses[i+1] = newPos.copy()
                        if i == n-2:
                            visitedPos.append(knotPoses[i+1].copy())

                        # another way would have been to have the second knot go to the old position
                        # of the first and have subsequent knots emulate the movement of the second
                        # i.e. diagonal if it had to go diagonal, assuming that the knot ahead of them
                        # was out of the range

    visitedPos = set([tuple(pos) for pos in visitedPos])
    
    return len(visitedPos)

def find_distance(p1, p2):
    diffH = p2[0] - p1[0]
    diffV = p2[1] - p1[1]

    return math.sqrt(diffH**2 + diffV**2)

filename = sys.argv[1]

print(count_visited_more_knots(filename, 10))