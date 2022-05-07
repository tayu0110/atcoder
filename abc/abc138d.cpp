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
const ld PI = acos(-1);
void dfs(int now, int par, vector<int> &res, vector<int> &p, Graph &t) {
  res[now] += p[now];
  for(auto e : t[now]) {
    if(e == par) continue;
    res[e] += res[now];
    dfs(e, now, res, p, t);
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  vector<int> p(n, 0);
  for(int i=0;i<q;i++) {
    int t, x;
    cin >> t >> x;
    t--;
    p[t] += x;
  }
  vector<int> res(n, 0);
  dfs(0, -1, res, p, t);
  for(int i=0;i<n;i++) {
    cout << res[i];
    if(i == n-1) cout << endl;
    else cout << " ";
  }
  return 0;
}
