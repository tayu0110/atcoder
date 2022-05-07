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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  vector<pii> p(q);
  for(int i=0;i<q;i++) {
    int l, r;
    cin >> l >> r;
    p[i] = {l, r};
  }
  vector<set<int>> t(n+1);
  for(int i=0;i<q;i++) {
    auto [l, r] = p[i];
    t[l].insert(r);
  }
  Graph g(n+10);
  for(int i=1;i<n+1;i++) {
    int prev = -1;
    for(auto e : t[i]) {
      if(prev < 0) {
        prev = e;
        g[i].push_back(e);
        continue;
      }
      g[prev+1].push_back(e);
      t[prev+1].insert(e);
      prev = e;
    }
  }
  queue<int> nt;
  nt.push(1);
  vector<bool> ck(n+10, false);
  while(!nt.empty()) {
    int now = nt.front();
    nt.pop();
    if(ck[now]) continue;
    ck[now] = true;
    for(auto e : g[now]) {
      if(ck[e+1]) continue;
      nt.push(e+1);
    }
  }
  if(ck[n+1]) {
    cout << "Yes" << endl;
  } else {
    cout << "No" << endl;
  }
  return 0;
}
