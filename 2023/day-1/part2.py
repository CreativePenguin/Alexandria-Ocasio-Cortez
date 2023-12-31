def part1(line):
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

def part2(line):
    numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
    str_to_num = {'zero': 0, 'one': 1, 'two': 2, 'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7, 'eight': 8, 'nine': 9,
                  '0': 0, '1': 1, '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7, '8': 8, '9': 9}
    first_digit = 0
    second_digit = 0
    for index in range(len(line)):
        num_found = False
        for num in numbers:
            potential_num = line[index:index + len(num)]
            if potential_num == num:
                first_digit = str_to_num[potential_num]
                num_found = True
                break
        if num_found:
            break

    for index in range(len(line[:-1]), 0, -1):
        num_found = False
        for num in numbers:
            potential_num = line[index - len(num):index]
            # if num == 'one':
            #     print(potential_num, num, potential_num == num)
            if potential_num == num:
                second_digit = str_to_num[potential_num]
                num_found = True
                break
        if num_found:
            break
    return first_digit * 10 + second_digit

    # for i in numbers:
    #     for ii in range(line - len(i)):
    #         if line[ii:len(i) + ii] == i:
    #             number_index[ii] = i

with open('input', 'r') as fi:
    sum = 0
    for i in fi.readlines():
        sum += part2(i)
    print(sum)
