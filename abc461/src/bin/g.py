from pulp import LpProblem, LpVariable, const, value, PULP_CBC_CMD

n, m = map(int, input().split())
edges = [tuple(map(int, input().split())) for _ in range(m)]

vars = []
for i in range(n):
    vars.append(LpVariable(f"x{i}", lowBound=0, cat=const.LpInteger))

m = LpProblem(sense=const.LpMaximize)
m += sum(vars)
for u, v in edges:
    m += vars[u-1] + vars[v-1] <= 2026

m.solve(PULP_CBC_CMD(msg=False))
print(list(map(lambda x: value(x), vars)))
print(sum(map(lambda x: int(value(x)), vars)))
