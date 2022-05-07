#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
struct UnionFind {
  vector<int> par;
  UnionFind(int n) : par(vector<int>(n, -1)) {}
  int root(int x) { return par[x] < 0 ? x : par[x] = root(par[x]); }
  bool isSame(int x, int y) { return root(x) == root(y); }
  int size(int x) { return -par[root(x)]; }
  bool merge(int x, int y) {
    int rx = root(x), ry = root(y);
    if(rx == ry) return false;
    if(par[rx] > par[ry]) swap(rx, ry);
    par[rx] += par[ry];
    par[ry] = rx;
    return true;
  }
};
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    int a, b;
    cin >> a >> b;
    p[i] = {a, b};
  }
  UnionFind uf(n+2);
  vector<tuple<double, int, int>> t;
  for(int i=0;i<n;i++) {
    auto [a, b] = p[i];
    t.push_back({100-b, i, n});
    t.push_back({b+100, i, n+1});
    for(int j=i+1;j<n;j++) {
      auto [na, nb] = p[j];
      double d = (b-nb) * (b-nb) + (a-na) * (a-na);
      t.push_back({sqrt(d), i, j});
    }
  }
  sort(t.begin(), t.end());
  for(auto [d, i, j] : t) {
    uf.merge(i, j);
    if(uf.isSame(n, n+1)) {
      cout << d / 2 << endl;
      return 0;
    }
  }
  return 0;
}
