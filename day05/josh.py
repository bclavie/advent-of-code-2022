a, b = open("input.txt").read().split("\n\n")
cx = [[j for j in i if j != " "] for i in zip(*[l[1::4] for l in a.split("\n")[:-1][::-1]])]
d = [list(map(int, l.split(" ")[1::2])) for l in b.strip().split("\n")]

def do(c, rev=0):
    for n, f, t in d:
        f = f-1
        tmp, c[f] = c[f][-n:], c[f][:-n]
        c[t-1].extend(tmp[::-1] if rev else tmp)
    print("".join([i[-1] for i in c]))

do([c[:] for c in cx], 1)
do(cx)
