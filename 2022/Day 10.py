import sys, re

def find_signal_strength(filename, cycle_stop):
    with open(filename) as ifile:
        cycle = 1
        X = 1
        for line in ifile:
            if cycle == cycle_stop:
                    return X*cycle
            elif line.find("noop") != -1:
                cycle += 1
            elif line.find("addx") != -1:
                val = eval(re.findall('\d+', line)[0])
                if line.find("-") != -1:
                    val = -val
                cycle += 1
                if cycle == cycle_stop:
                    return X*cycle
                cycle += 1
                X += val

def find_X(filename, cycle_stop):
    with open(filename) as ifile:
        cycle = 1
        X = 1
        for line in ifile:
            if cycle == cycle_stop:
                    return X
            elif line.find("noop") != -1:
                cycle += 1
            elif line.find("addx") != -1:
                val = eval(re.findall('\d+', line)[0])
                if line.find("-") != -1:
                    val = -val
                cycle += 1
                if cycle == cycle_stop:
                    return X
                cycle += 1
                X += val


def draw_CRT(filename, size = (40,6)):
    screen = []
    cycle = 1
    for i in range(size[1]):
        row = ""
        for j in range(size[0]):
            X = find_X(filename, cycle)
            dist = abs(j - X)

            if dist <= 1:
                row+='#'
            else:
                row+='.'

            cycle+=1
        screen.append(row)
    
    for s in screen:
        print(s)




filename = sys.argv[1]

# sum = 0

# for i in range(20, 221, 40):
#     sum += find_signal_strength(filename, cycle_stop=i)

# print(sum)

draw_CRT(filename)
