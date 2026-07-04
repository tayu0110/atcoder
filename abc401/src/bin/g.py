import pulp
import numpy as np
import math

n = int(input())
s = [tuple(map(int, input().split())) for _ in range(n)]
g = [tuple(map(int, input().split())) for _ in range(n)]


cost = np.empty((n, n))
for i, (sx, sy) in enumerate(s):
    for j, (gx, gy) in enumerate(g):
        dist = (sx - gx) * (sx - gx) + (sy - gy) * (sy - gy)
        cost[i, j] = math.sqrt(dist)


edges = np.array(plup.LpVariable.matrix("edge", (range(n), range(n)), cat="Binary"))
