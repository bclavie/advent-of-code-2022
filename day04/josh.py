a = [[int(i) for e in p.split(",") for i in e.split("-")] for p in open("input.txt").read().strip().split("\n")]
print(sum(map(lambda x: (x[0]>=x[2] and x[1]<=x[3]) or (x[2]>=x[0] and x[3]<=x[1]), a)))
print(sum(map(lambda x: (x[0]>=x[2] and x[0]<=x[3]) or (x[2]>=x[0] and x[2]<=x[1]), a)))
