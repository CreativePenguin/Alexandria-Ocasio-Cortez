with open('input', 'r') as fi:
    sum = 0
    for i in fi.readlines():
        sum += part1(i)
    print(sum)
