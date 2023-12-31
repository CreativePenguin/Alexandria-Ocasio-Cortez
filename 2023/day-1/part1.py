def solution(line):
    first_digit = 0
    last_digit = 0
    for i in line:
        try:
            first_digit = int(i)
            break
        except ValueError:
            continue
    for i in reversed(line):
        try:
            last_digit = int(i)
            break
        except ValueError:
            continue
    val = first_digit * 10 + last_digit
    return val

with open('input', 'r') as fi:
    sum = 0
    for i in fi.readlines():
        sum += solution(i)
    print(sum)
