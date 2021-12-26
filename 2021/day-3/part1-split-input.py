with open('input') as rf:
    with open('split-input', 'w+') as wf:
        for line in rf.readlines():
            buf = line[0]
            for letter in line[1:]:
                buf += ' '
                buf += letter
            buf += '\n'
            wf.write(buf)
