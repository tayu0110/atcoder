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
vector<vector<int>> v;
vector<int> x;
void rec(int now, int par, Graph &t) {
  for(auto to : t[now]) {
    if(to == par) continue;
    rec(to, now, t);
  }
  if(par < 0) return;
  priority_queue<int> nt;
  for(auto e : v[now]) nt.push(e);
  for(auto e : v[par]) nt.push(e);
  vector<int> tmp;
  while(!nt.empty()) {
    tmp.push_back(nt.top()), nt.pop();
    if(tmp.size() == 20) break;
  }
  tmp.swap(v[par]);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  x.assign(n, 0);
  for(int i=0;i<n;i++) cin >> x[i];
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  v.assign(n, vector<int>(0));
  for(int i=0;i<n;i++) v[i].push_back(x[i]);
  rec(0, -1, t);
  while(q--) {
    int l, k;
    cin >> l >> k;
    l--; k--;
    cout << v[l][k] << endl;
  }
  return 0;
}
