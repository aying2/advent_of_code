import functools

def compare(left: list|int, right: list|int) -> bool|None:
    if type(left) == int and type(right) == int:
        if left < right:
            return True
        elif left > right:
            return False
        else:
            return None
    elif type(left) == list and type(right) == list:
        for i in range(0, min(len(left), len(right))):
            res = compare(left[i], right[i])
            if res is not None:
                return res
        if len(left) < len(right):
            return True
        elif len(left) > len(right):
            return False
        else:
            return None

    elif type(left) == int and type(right) == list:
        return compare([left], right)
    else:
        return compare(left, [right])

def compare_int(left: list|int, right: list|int) -> int:
    if type(left) == int and type(right) == int:
        if left < right:
            return -1
        elif left > right:
            return 1
        else:
            return 0
    elif type(left) == list and type(right) == list:
        for i in range(0, min(len(left), len(right))):
            res = compare_int(left[i], right[i])
            if res != 0:
                return res
        if len(left) < len(right):
            return -1
        elif len(left) > len(right):
            return 1
        else:
            return 0

    elif type(left) == int and type(right) == list:
        return compare_int([left], right)
    else:
        return compare_int(left, [right])

def isList(s:str) -> bool:
    if s[0] == "[" and s[-1] == "]":
        return True
    else:
        return False

def isInt(s:str) -> bool:
    try:
        int(s)
        return True
    except:
        return False

def str_to_list(s: str) -> list:
    """
    Extracts list of ints and lists from string
    
        parameters:
            s: a string
        returns:
            list
    
    pre: s begins with [ and end with ]
        lists in string only contain ints
    """
    if not isList(s):
        breakpoint()
        raise Exception("s does not begin with [ and end with ]")

    # remove top level brackets
    s = s[1:-1]

    # collect values at the top level of list
    ret = []

    # collect sublists for recursion
    sub = ""

    # to handle numbers with multiple digits
    digits = ""

    # is 0 when next topmost level open bracket is closed, signalling recursion of substring
    unclosed = 0

    for i,c in enumerate(s):
        match c:
            case "[":
                # start adding to a substring
                unclosed += 1
                sub += c
            case "]":
                # close one of the open brackets
                unclosed -= 1
                sub += c

                # if back at the top level, append recursion
                if unclosed == 0:
                    ret.append(str_to_list(sub))
                    sub = ""                                  
            case _:
                if unclosed == 0:
                    if isInt(c):       
                        digits += c    

                    # end of int is signalled either by comma or end of line
                    # make sure that digits has values, otherwise comma follows a list                         
                    if len(digits) > 0 and c == "," or i == len(s) - 1:
                        ret.append(int(digits))
                        digits = ""
                else:
                    # if not on top level, put it straight in the substring for the comma separated ints
                    # to be processed in the recursion
                    sub += c 
    return ret

def parse_pairs(filename:str) -> list:
    ordered_idx = []
    with open(filename) as file:
        prev = []
        for i, line in enumerate(file):
            line = line.strip("\n")
            if i % 3 == 0:
                prev = str_to_list(line)
            elif i % 3 == 1:
                line = str_to_list(line)
                res = compare(prev, line)
                
                if res:
                    pair_idx = i//3 + 1
                    ordered_idx.append(pair_idx)
    
    return ordered_idx

def parse_sort(filename:str) -> int:
    with open(filename) as file:
        lines = []
        for i, line in enumerate(file):
            line = line.strip("\n")
            if line != "":
                lines.append(str_to_list(line))
        lines.append([[2]])
        lines.append([[6]])
        sort = sorted(lines, key=functools.cmp_to_key(compare_int))

        return (sort.index([[2]])+1) * (sort.index([[6]])+1)
    

if __name__ == "__main__":
    # res = parse_pairs("input13.txt")
    print(parse_sort("input13.txt"))

    # left = str_to_list("[[2,3,4]]")
    # right = str_to_list("[4]")

    # print(compare(left, right))