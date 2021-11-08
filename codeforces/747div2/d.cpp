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

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};
using weightedGraph = vector<vector<Edge>>;
struct UnionFind {
  vector<int> par;
  UnionFind(int n) : par(vector<int>(n, -1)) {}
  int root(int x) {
    if(par[x] < 0) return x;
    return par[x] = root(par[x]);
  }
  bool merge(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    if(rx == ry) return false;
    if(par[rx] > par[ry]) swap(rx, ry);
    par[rx] += par[ry];
    par[ry] = rx;
    return true;
  }
  bool isSame(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    return rx == ry;
  }
  int size(int x) {
    return -par[root(x)];
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t; cin >> t;
  vector<int> res;
  while(t--) {
    int n, m;
    cin >> n >> m;
    UnionFind uf(n);
    weightedGraph g(n);
    for(int i=0;i<m;i++) {
      int a, b;
      cin >> a >> b;
      a--;b--;
      string s;
      cin >> s;
      uf.merge(a, b);
      g[a].push_back(Edge(b, (s[0] == 'c')));
      g[b].push_back(Edge(a, (s[0] == 'c')));
    }
    set<int> ck;
    vector<int> d(n, -1);
    queue<pii> nt;
    bool f = true;
    int ans = 0;
    for(int i=0;i<n;i++) {
      int root = uf.root(i);
      if(ck.find(root) != ck.end()) continue;
      ck.insert(root);
      nt.push({i, 1});
      int one = 0;
      while(!nt.empty()) {
        auto [now, nd] = nt.front();
        nt.pop();
        if(d[now] >= 0) continue;
        d[now] = nd;
        if(nd) one++;
        for(auto [to, w] : g[now]) {
          if(d[to] >= 0) {
            if(w && d[to] != nd) f = false;
            else if(!w && d[to] == nd) f = false;
            continue;
          }
          if(w) nt.push({to, nd});
          else nt.push({to, (nd+1)%2});
        }
      }
      int zero = uf.size(root) - one;
      ans += max(one, zero);
    }
    if(!f) res.push_back(-1);
    else res.push_back(ans);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
