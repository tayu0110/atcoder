from pulp import *

t = int(input())
res = []

for _ in range(t):
    a = list(map(int, input().split()))
    p = list(map(int, input().split()))
    
    w = 0
    sum = 0
    for i in range(5):
        w += a[i] * (i + 1)
        sum += a[i]
    
    if w >= 3 * sum:
        res.append(0)
        continue
    
    s = 3 * sum - w
    m = LpProblem()
    x = LpVariable('x', lowBound=0, upBound=10000000000, cat=const.LpInteger)
    y = LpVariable('y', lowBound=0, upBound=10000000000, cat=const.LpInteger)
    
    m += p[4] * x + p[3] * y
    m += 2 * x + y >= s
    m.solve(PULP_CBC_CMD(msg=0))
    
    x = int(value(x))
    y = int(value(y))
    res.append(x * p[4] + y * p[3])

print(*res, sep='\n')