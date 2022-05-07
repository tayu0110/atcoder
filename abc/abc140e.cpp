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
  int n;
  cin >> n;
  vector<int> p(n);
  for(int i=0;i<n;i++) cin >> p[i];
  vector<pll> t(n);
  for(int i=0;i<n;i++) t[i] = {p[i], i};
  sort(t.begin(), t.end(), greater<pll>());
  set<int> ck;
  ck.insert(-1);
  ck.insert(n);
  ck.insert(t[0].second);
  ll ans = 0;
  for(int i=1;i<n;i++) {
    auto [nt, idx] = t[i];
    int prev = -inf, pp = -inf;
    int nxt = inf, nn = inf;
    auto it = ck.lower_bound(idx);
    nxt = *it;
    it++;
    if(it != ck.end()) nn = *it;
    it--;
    it--;
    prev = *it;
    if(it != ck.begin()) {
      it--;
      pp = *it;
    }
    if(pp != -inf) {
      ans += nt * (prev - pp) * (nxt - idx);
    }
    if(nn != inf) {
      ans += nt * (idx - prev) * (nn - nxt );
    }
    ck.insert(idx);
  }
  cout << ans << endl;
  return 0;
}
