def parse_game(line):
    game_num = int(line.split(':')[0].split(' ')[1])
    rounds = line.split(';')
    rounds[0] = rounds[0].split(':')[1]
    rounds = [i.strip() for i in rounds]
    round_colors = []
    valid_game = True
    for roundi in rounds:
        round_results = {}
        colors = roundi.split(',')
        for i in colors:
            num, color = i.strip().split(' ')
            round_results[color] = int(num)
        round_colors.append(round_results)
    return round_colors

def part1(line):
    round_results = parse_game(line)
    print(round_results)
    for i in round_results:
        if i.get('red', 0) > 12 or i.get('green', 0) > 13 or i.get('blue', 0) > 14:
            valid_game = False
            break
    return game_num if valid_game else 0

def part2(line):
    round_results = parse_game(line)
    min_cubes = round_results[0]
    for i in round_results:
        for key, value in i.items():
            if min_cubes.get(key, 0) < value:
                min_cubes[key] = value
    power = 1
    for i in min_cubes.values():
        power *= i
    return power

# with open('test_input', 'r') as fi:
with open('input', 'r') as fi:
    file_sum = 0
    # part1(fi.readline())
    for i in fi.readlines():
        file_sum += part2(i)
    print(file_sum)
