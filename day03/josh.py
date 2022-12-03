rs = [[o - 96 if o > 96 else o - 38 for o in map(ord, r)] for r in open("input.txt").read().strip().split("\n")]
print(sum([max(set(r[:(len(r)//2)]).intersection(r[len(r)//2:])) for r in rs]))
print(sum(max(set.intersection(*map(set,g))) for g in list(zip(rs[::3],rs[1::3],rs[2::3]))))