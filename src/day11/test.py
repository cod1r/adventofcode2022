class Monkey:
    def __init__(self):
        self.inspected = 0
        self.id = None
        self.items = None
        self.test = None
        self.op = None
        self.yes = None
        self.no = None
monkeys = []
with open("input.txt", "r") as I:
    lines = list(map(lambda x: x.strip(), I.readlines()))
    m = Monkey()
    for line in lines:
        if "Monkey" in line:
            m.id = line[:-1]
        elif "Starting items" in line:
            items = list(map(int, line[line.index(':') + 2:].split(',')))
            m.items = items
        elif "Test" in line:
            m.test = int(line.split(' ')[-1])
        elif "true" in line:
            m.yes = int(line.split(' ')[-1])
        elif "false" in line:
            m.no = int(line.split(' ')[-1])
        elif "Operation" in line:
            m.op = line
        elif len(line) == 0:
            monkeys.append(m)
            m = Monkey()
    monkeys.append(m)
for i in range(10000):
    for m in range(len(monkeys)):
        monkey = monkeys[m]
        monkey.inspected += len(monkey.items)
        monkey.items.reverse()
        while len(monkey.items) > 0:
            item = monkey.items.pop()
            if "+" in monkey.op:
                if ord('0') <= ord(monkey.op[-1]) <= ord('9'):
                    item += int(monkey.op.split(' ')[-1])
                else:
                    item += item
            elif "*" in monkey.op:
                if ord('0') <= ord(monkey.op[-1]) <= ord('9'):
                    item *= int(monkey.op.split(' ')[-1])
                else:
                    item = item ** 2
            item %= 2 * 7 * 3 * 17 * 11 * 19 * 5 * 13
            #item %= 23 * 19 * 13 * 17
            if (item % monkey.test) == 0:
                monkeys[monkey.yes].items.append(item)
            else:
                monkeys[monkey.no].items.append(item)
monkeys.sort(key=lambda a: a.inspected)
print(monkeys[-2].inspected * monkeys[-1].inspected)
for m in monkeys:
    print(m.inspected)
