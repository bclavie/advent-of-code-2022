d = [[c[0] - c[1] for c in list(zip(map(ord, d.split(" ")), (65, 88)))] for d in open("input.txt").read().strip().split("\n")]
print(sum(map(lambda x: (x[1] - x[0] + 1) % 3 * 3 + x[1] + 1, d)))
print(sum(map(lambda x: ((x[1] - 1) + x[0]) % 3 + 1 + x[1] * 3, d)))