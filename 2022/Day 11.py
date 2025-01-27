from io import TextIOBase
import sys, re
import copy

class Monkey:
    def __init__(self, num,  items, operation, test, passt, passf):
        self.num = num
        self.items = items
        self.operation = operation
        self.test = test
        self.passt = passt
        self.passf = passf
        self.count = 0

    def __str__(self):
        sys.set_int_max_str_digits(0)
        s = "num: {} \n items: {} \n operation: {} \n test: {} \n passt: {} \n passf: {} \n count: {}".format(
            self.num, self.items, self.operation, self.test, self.passt, self.passf, self.count)
        return s


def parse(filename: str) -> list:
    comp = re.compile(r"\d+")
    with open(filename) as file:
        monkeys = []
        
        params = []

        for i, line in enumerate(file):
            

            match i % 7:
                case 1:
                    m = comp.findall(line)
                    params.append(m)
                case 2:
                    m = re.search(r"([*+]) (\d+|old)", line)
                    if m:
                        params.append(m.groups())
                case _:
                    m = comp.search(line)
                    if m:
                        params.append(m.group())
                
            if i % 7 == 5:
                monkey = Monkey(params[0], params[1], params[2], params[3], params[4], params[5])
                monkeys.append(monkey)
                params.clear()
            
        return monkeys

def throw (item: int, to_monkey: Monkey) -> None:
    to_monkey.items.append(item)

def simulate (monkeys: list, num_iterations: int, mod_base:int) -> None:
    for n in range(num_iterations):
        for monkey in monkeys:
            for item in monkey.items:
                item = int(item)
                if monkey.operation[0] == "*":
                    if monkey.operation[1] == "old":
                        item *= item
                    else:
                        item *= int(monkey.operation[1])
                elif monkey.operation[0] == "+":
                    item += int(monkey.operation[1])

                # item //= 3

                item %= mod_base            

                if item % int(monkey.test) == 0:
                    throw(item, monkeys[int(monkey.passt)])
                else:
                    throw(item, monkeys[int(monkey.passf)])
                monkey.count += 1
            monkey.items.clear()

if __name__== "__main__":

    monkeys = parse("input11.txt")

    mod_base = 1
    for monkey in monkeys:
        print(monkey)
        mod_base *= int(monkey.test)

    simulate(monkeys, 10000, mod_base)

    counts = []
    for monkey in monkeys:
        print(monkey)
        counts.append(monkey.count)

    counts = sorted(counts)
    print(counts)
    print(counts[len(counts)-1] * counts[len(counts) - 2])

