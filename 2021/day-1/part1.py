counter = 0
with open('input') as f:
    x = list(map(lambda a: int(a), f.readlines()))
    prev = x.pop(0)
    for line in x:
        if prev < line:
            counter += 1
        prev = line
print(counter)
