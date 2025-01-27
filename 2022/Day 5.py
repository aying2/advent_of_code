import sys
import re

def parse_initial(filename, n):
    with open(filename) as ifile:
        stacks = {k: [] for k in range(1, n+1)}
        for line in ifile:
            count = 1
            for i, c  in enumerate(line):
                if (i-1)%4 == 0:
                    # until row indicating stack numbers
                    if c.isnumeric():
                        # flip so the top is at end
                        for key in stacks:
                            stacks[key].reverse()
                        return stacks
                    elif c.isalpha():
                        stacks[count].append(c)
                    count+=1

def make_moves(filename, stacks):
    with open(filename) as ifile:
        for line in ifile:
            if(line.find("move") != -1):
                move = re.findall('\d+', line)
                move = [eval(i) for i in move]
                removed = []
                for i in range(0, move[0]):
                    removed.append(stacks[move[1]].pop(len(stacks[move[1]])-1))
                # print(removed)
                # apparently .removed() doesn't return so cannot be used in index
                stacks[move[2]].extend(removed[::-1])

def get_tops(stacks):
    tops = ""
    for key, val in stacks.items():
        tops+=val[len(val)-1]
    return tops
                
                

filename = sys.argv[1]
stacks = parse_initial(filename, 9)
make_moves(filename, stacks)
print(stacks)
print(get_tops(stacks))

