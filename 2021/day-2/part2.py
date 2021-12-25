with open("input") as f:
    aim = 0
    horiz = 0
    depth = 0
    for line in f.readlines():
        direct = line.split(' ')[0]
        dist = int(line.split(' ')[1])
        if direct == 'up':
            aim -= dist
        if direct == 'down':
            aim += dist
        if direct == 'forward':
            horiz += dist
            depth += aim * dist
    print(horiz)
    print(depth)
    print(horiz * depth)
