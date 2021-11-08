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
struct UnionFind {
  vector<int> par;
  vector<map<int, int>> c;
  UnionFind(int n) : par(vector<int>(n, -1)), c(vector<map<int, int>>(n)) {}
  void init(int idx, int val) {
    c[idx][val] = 1;
  }
  int root(int x) {
    if(par[x] < 0) return x;
    return par[x] = root(par[x]);
  }
  bool merge(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    if(rx == ry) return false;
    if(c[rx].size() < c[ry].size()) c[rx].swap(c[ry]);
    for(auto e : c[ry]) c[rx][e.first] += e.second;
    if(par[rx] > par[ry]) {
      swap(rx, ry);
      c[rx].swap(c[ry]);
    }
    par[rx] += par[ry];
    par[ry] = rx;
    return true;
  }
  bool isSame(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    return rx == ry;
  }
  int size(int x, int cls) {
    // return -par[root(x)];
    int rt = root(x);
    return c[rt][cls];
  }
};
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  UnionFind uf(n);
  for(int i=0;i<n;i++) {
    int k;
    cin >> k;
    k--;
    uf.init(i, k);
  }
  while(q--) {
    int k, a, b;
    cin >> k >> a >> b;
    a--;b--;
    if(k == 1) {
      uf.merge(a, b);
    } else {
      cout << uf.size(a, b) << endl;
    }
  }
  return 0;
}
