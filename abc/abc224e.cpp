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
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w, n;
  cin >> h >> w >> n;
  using T = tuple<int, int, ll, int>;
  vector<T> p(n);
  map<ll, int> cv;
  for(int i=0;i<n;i++) {
    int r, c;
    ll a;
    cin >> r >> c >> a;
    r--;c--;
    p[i] = {r, c, a, i};
    cv[a] = 0;
  }
  int cnt = 0;
  for(auto &[f, s] : cv) s = cnt++;
  for(auto &[r, c, a, i] : p) a = cv[a];
  sort(p.begin(), p.end(), [](T lhs, T rhs) {
    auto [lr, lc, la, li] = lhs; auto [rr, rc, ra, ri] = rhs;
    return la > ra;
  });
  vector<int> vmx(w), hmx(h);
  vector<int> ans(n);
  queue<T> lazy;
  int nowa = -1;
  for(int i=0;i<n;i++) {
    auto [r, c, a, id] = p[i];
    if(a != nowa) {
      while(!lazy.empty()) {
        auto [nr, nc, na, ni] = lazy.front(); lazy.pop();
        vmx[nc] = max(vmx[nc], ans[ni]+1);
        hmx[nr] = max(hmx[nr], ans[ni]+1);
      }
      nowa = a;
    }
    ans[id] = max(vmx[c], hmx[r]);
    lazy.push(p[i]);
  }
  for(auto e : ans) cout << e << endl;
  return 0;
}
