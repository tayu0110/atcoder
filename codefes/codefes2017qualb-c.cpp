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
bool dfs(int now, vector<int> &v, Graph &t) {
  bool f = true;
  for(auto to : t[now]) {
    if(v[to] < 0) {
      v[to] = (v[now]+1) % 2;
      f &= dfs(to, v, t);
    } else {
      if(v[to] == v[now]) return false;
    }
  }
  return f;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n, m;
  cin >> n >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  vector<int> v(n, -1);
  v[0] = 0;
  bool f = dfs(0, v, t);
  if(f) {
    ll b = accumulate(v.begin(), v.end(), 0);
    ll w = n - b;
    cout << b * w - m << endl;
  } else {
    cout << n * (n-1) / 2 - m << endl;
  }
  return 0;
}
