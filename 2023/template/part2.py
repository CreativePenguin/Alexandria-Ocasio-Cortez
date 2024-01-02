with open('input', 'r') as fi:
    sum = 0
    for i in fi.readlines():
        sum += part2(i)
    print(sum)
