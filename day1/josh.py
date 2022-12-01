elfs_cals = sorted([sum(map(int, elf.split("\n"))) for elf in open("input.txt").read().strip().split("\n\n")])[::-1]
print(elfs_cals[0], sum(elfs_cals[:3]))
