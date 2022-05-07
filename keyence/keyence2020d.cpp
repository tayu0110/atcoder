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
int calc(vector<int> &t) {
  int n = t.size();
  int res = 0;
  for(int i=0;i<n;i++) {
    for(int j=0;j<i;j++) {
      if(t[i] < t[j]) res++;
    }
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  int ans = n * n * n;
  for(int i=0;i<(1<<n);i++) {
    set<tuple<int, int, int>> ck;
    for(int j=0;j<n;j++) {
      if(i & (1<<j)) {
        if(j % 2) ck.insert({a[j], 1, j});
        else ck.insert({a[j], 0, j});
      } else {
        if(j % 2) ck.insert({b[j], 0, j});
        else ck.insert({b[j], 1, j});
      }
    }
    bool f = true;
    vector<int> t;
    for(int j=0;j<n;j++) {
      auto [mn, _, __] = *ck.begin();
      for(auto it=ck.begin(); it!=ck.end(); it++) {
        auto [now, bin, idx] = *it;
        if(now > mn) {
          f = false;
          break;
        }
        if(bin == j % 2) {
          t.push_back(idx);
          ck.erase(it);
          break;
        }
      }
      if(t.size() != j+1) f = false;
      if(!f) break;
    }
    if(!f) continue;
    // DEBUG(i);
    // print_with_space(t);
    ans = min(ans, calc(t));
  }
  if(ans == n * n * n) cout << -1 << endl;
  else cout << ans << endl;
  return 0;
}
