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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
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
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
  UnionFind uf(h+w);
  for(int i=0;i<h;i++) cin >> s[i];
  s[0][0] = s[0][w-1] = s[h-1][0] = s[h-1][w-1] = '#';
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if(s[i][j] == '#') uf.merge(i, h+j);
    }
  }
  set<int> r, c;
  int rs = 0, cs = 0;
  for(int i=0;i<h;i++) {
    int root = uf.root(i);
    if(r.find(root) == r.end()) {
      r.insert(root);
      rs++;
    }
  }
  for(int j=0;j<w;j++) {
    int root = uf.root(h+j);
    if(c.find(root) == c.end()) {
      c.insert(root);
      cs++;
    }
  }
  cout << min(rs-1, cs-1) << endl;
  return 0;
}
