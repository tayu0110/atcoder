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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

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
  int n, m;
  cin >> n >> m;
  vector<int> d(n);
  UnionFind uf(n);
  for(int i=0;i<n;i++) cin >> d[i];
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--; b--;
    d[a]--; d[b]--;
    uf.merge(a, b);
  }
  if(*min_element(d.begin(), d.end()) < 0) {
    cout << -1 << endl;
    return 0;
  }
  {
    ll rem = accumulate(d.begin(), d.end(), 0LL);
    if(rem % 2 != 0 || rem / 2 != n - m - 1) {
      cout << -1 << endl;
      return 0;
    }
  }
  map<int, int> mp;
  vector<vector<int>> v(n);
  for(int i=0;i<n;i++) {
    if(!d[i]) continue;
    int root = uf.root(i);
    mp[root] += d[i];
    for(int j=0;j<d[i];j++) v[root].push_back(i);
  }
  queue<int> last_one;
  vector<vector<int>> large;
  for(auto [f, s] : mp) {
    if(s == 1) last_one.push(v[f].back());
    else large.push_back(v[f]);
  }
  vector<pii> ans;
  for(auto e : large) {
    for(int i=0;i<e.size()-1;i++) {
      if(last_one.empty()) {
        cout << -1 << endl;
        return 0;
      }
      int lo = last_one.front(); last_one.pop();
      ans.push_back({lo, e[i]});
      uf.merge(lo, e[i]);
    }
    last_one.push(e.back());
  }
  if(last_one.size() != 2) {
    cout << -1 << endl;
    return 0;
  }
  int l = last_one.front(); last_one.pop();
  int r = last_one.front(); last_one.pop();
  ans.push_back({l, r});
  uf.merge(l, r);
  if(uf.size(0) != n) {
    cout << -1 << endl;
    return 0;
  }
  for(auto [l, r] : ans) cout << l+1 << " " << r+1 << endl;
  return 0;
}
