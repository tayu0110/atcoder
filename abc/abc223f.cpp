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
const ld PI = 3.141592653589793238462643383;
pii op(pii a, pii b) {
  return make_pair(min(a.first, a.second + b.first), a.second + b.second);
}
pii e() {
  return make_pair(0, 0);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  string s;
  cin >> s;
  vector<pii> v(n);
  for(int i=0;i<n;i++) {
    if(s[i] == '(') v[i] = {0, 1};
    else v[i] = {-1, -1};
  }
  segtree<pii, op, e> st(v);
  while(q--) {
    int t, l, r;
    cin >> t >> l >> r;
    l--;
    if(t == 1) {
      r--;
      swap(v[l], v[r]);
      st.set(l, v[l]);
      st.set(r, v[r]);
    } else {
      auto [mn, sum] = st.prod(l, r);
      if(mn >= 0 && sum == 0) cout << "Yes" << endl;
      else cout << "No" << endl;
    }
  }
  return 0;
}
