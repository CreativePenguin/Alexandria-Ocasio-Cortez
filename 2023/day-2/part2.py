def parse_game(line):
    game_num = int(line.split(':')[0].split(' ')[1])
    rounds = line.split(';')
    rounds[0] = rounds[0].split(':')[1]
    rounds = [i.strip() for i in rounds]
    valid_game = True
    for roundi in rounds:
        round_results = {}
        colors = roundi.split(',')
        for i in colors:
            num, color = i.strip().split(' ')
            round_results[color] = int(num)

def part1(line):
    game_num = int(line.split(':')[0].split(' ')[1])
    rounds = line.split(';')
    rounds[0] = rounds[0].split(':')[1]
    rounds = [i.strip() for i in rounds]
    valid_game = True
    for roundi in rounds:
        round_results = {}
        colors = roundi.split(',')
        for i in colors:
            num, color = i.strip().split(' ')
            round_results[color] = int(num)
        if round_results.get('red', 0) > 12 or round_results.get('green', 0) > 13 or round_results.get('blue', 0) > 14:
            valid_game = False
            break
    return game_num if valid_game else 0

def part2(line):
    

with open('input', 'r') as fi:
    file_sum = 0
    # part1(fi.readline())
    for i in fi.readlines():
        file_sum += part1(i)
    print(file_sum)
