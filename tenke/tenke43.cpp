#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
pii goal;
int h, w;
set<pii> ck;
vector<vector<int>> memo;
int dfs(pii pp, pii p, pii now, vector<string>& s) {
  // cout << now.first << " " << now.second << endl;
  bool f = (pp.first != p.first || p.first != now.first) && (pp.second != p.second || p.second != now.second);
  if(now == goal) return (f ? 1 : 0);
  if(memo[now.second][now.first] >= 0) return memo[now.second][now.first] + (f ? 1 : 0);
  if(ck.find(now) != ck.end()) return inf;
  ck.insert(now);
  int res = inf;
  vector<int> dx = {1, 0, -1, 0}, dy = {0, 1, 0, -1};
  for(int i=0;i<4;i++) {
    int x = now.first + dx[i];
    int y = now.second + dy[i];
    if(x < 0 || x >= w || y < 0 || y >= h) continue;
    if(s[y][x] == '#') continue;
    res = min(res, dfs(p, now, {x, y}, s));
  }
  memo[now.second][now.first];
  if(f) res++;
  return res;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> h >> w;
  int rs, cs, rt, ct;
  cin >> rs >> cs >> rt >> ct;
  rs--;cs--;rt--;ct--;
  goal = {ct, rt};
  pii start = {cs, rs};
  memo.assign(h, vector<int>(w, -1));
  vector<string> s(h);
  for(int i=0;i<h;i++) {
    cin >> s[i];
  }
  cout << dfs(start, start, start, s) << endl;
  return 0;
}
