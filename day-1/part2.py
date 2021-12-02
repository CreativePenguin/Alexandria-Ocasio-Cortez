counter = 0
with open('input') as f:
    x = list(map(lambda a: int(a), f.readlines()))
    prev = x[0] + x[1] + x[2]
    prev = x.pop(0)
    for line in range(len(x) - 2):
        nextthree = x[line] + x[line + 1] + x[line + 2]
        if prev < nextthree:
            counter += 1
        prev = nextthree
print(counter)
