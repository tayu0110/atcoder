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

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

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
vector<string> s;
int n, k;
vector<int> dx = {-1, 0, 1, 0};
vector<int> dy = {0, -1, 0, 1};
bool check(vector<int> &t) {
  vector<string> p;
  p.assign(n, "");
  pii start = {-1, -1};
  for(int j=0;j<n;j++) {
    int e = t[j];
    for(int i=0;i<n;i++) {
      if(e & (1<<i)) {
        p[j].push_back('1');
        start = {j, i};
      } else p[j].push_back('0');
    }
  }
  // for(auto e : p) cout << e << endl;
  // cout << endl;
  vector<vector<bool>> ck(n, vector<bool>(n, false));
  queue<pii> nt;
  nt.push(start);
  while(!nt.empty()) {
    auto now = nt.front();
    nt.pop();
    int y = now.first;
    int x = now.second;
    if(ck[y][x]) continue;
    ck[y][x] = true;
    for(int i=0;i<4;i++) {
      int nx = x + dx[i];
      int ny = y + dy[i];
      if(nx < 0 || nx >= n || ny < 0 || ny >= n) continue;
      if(p[ny][nx] != '1') continue;
      nt.push({ny, nx});
    }
  }
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    if(p[i][j] == '1') {
      if(!ck[i][j]) return false;
    }
  }
  return true;
}
ll dfs(int par, int level, int count, vector<int>& t) {
  int res = 0;
  for(int i=0;i<(1<<n);i++) {
    int cnt = 0;
    bool f = false;
    if(level == 0) f = true;
    for(int j=0;j<n;j++) {
      if(i & (1<<j)) {
        if(s[level][j] == '#') goto bad;
        cnt++;
      }
    }
    if(!(par & i)) f = false;
    if(par == 0) f = true;
    if(!f) continue;;
    cnt += count;
    if(cnt > k) continue;
    if(cnt == k) {
      t.push_back(i);
      while(t.size() < n) t.push_back(0);
      bool g = check(t);
      if(g) res++;
      while(t.size() > level) t.pop_back();
    } else {
      if(level+1 == n) continue;
      t.push_back(i);
      res += dfs(i, level+1, cnt, t);
      t.pop_back();
    }
bad:continue;
  }
  return res;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n >> k;
  s.assign(n, "");
  for(int i=0;i<n;i++) cin >> s[i];
  vector<int> t;
  int ans = dfs(-1, 0, 0, t);
  cout << ans << endl;
  return 0;
}
