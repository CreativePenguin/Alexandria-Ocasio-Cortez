with open('input') as f:
    binarr = []
    for i in f.readlines()[0]:
        binarr.append(0)
    for line in f.readlines():
        for i in range(line):
            binarr[i] += line[i]
    print(binarr)
