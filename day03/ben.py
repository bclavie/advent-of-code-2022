input_lines = open("input.txt").read().splitlines() 

def get_letter_value(letter):
    if letter.islower():
        reference = ord('a') - 1
    else:
        reference = ord('A') - 27
    return ord(letter) - reference

part1 = sum(get_letter_value(x) for line in input_lines for x in set(line[:len(line) // 2]) & set(line[len(line) // 2:]))
print(f"Part 1 result: {part1}")
part2 = sum([get_letter_value(list(x)[0]) for x in [set(input_lines[i]) & set(input_lines[i + 1]) & set(input_lines[i + 2]) for i in range(0, len(input_lines), 3)]])
print(f"Part 2 result: {part2}")
