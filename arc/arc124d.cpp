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
  int n, m;
  cin >> n >> m;
  int t = n + m;
  vector<int> p(t);
  UnionFind uf(t);
  for(int i=0;i<t;i++) {
    cin >> p[i];
    p[i]--;
    uf.merge(i, p[i]);
  }
  int k = 0;
  map<int, int> mp;
  set<int> ck;
  for(int i=0;i<t;i++) {
    int root = uf.root(i);
    if(ck.find(root) == ck.end()) k++;
    ck.insert(root);
    if(uf.size(root) < 2) continue;
    if(i < n || p[i] < n) mp[root] |= 1 << 0;
    if(i >= n || p[i] >= n) mp[root] |= 1 << 1;
  }
  int r = 0, b = 0;
  for(auto [f, s] : mp) {
    if(s == 1) r++;
    if(s == 2) b++;
  }
  cout << t - k + 2 * max(r, b) << endl;
  return 0;
}
