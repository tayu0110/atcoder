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
  int n;
  cin >> n;
  using T = tuple<int, int, ll>;
  vector<T> p(n);
  int sum = 0;
  for(int i=0;i<n;i++) {
    int w, s;
    ll v;
    cin >> w >> s >> v;
    p[i] = {w, s, v};
    sum += w;
  }
  sort(p.begin(), p.end(), [](T l, T r) {
    auto [lw, ls, lv] = l;
    auto [rw, rs, rv] = r;
    return min(ls, rs-lw) > min(rs, ls-rw);
  });
  vector<ll> dp(sum+1, -1);
  dp[0] = 0;
  for(int i=0;i<n;i++) {
    auto [w, s, v] = p[i];
    for(int j=min(s, sum);j>=0;j--) {
      if(dp[j] < 0) continue;
      int nj = j + w;
      if(sum < nj) continue;
      dp[nj] = max(dp[nj], dp[j] + v);
    }
  }
  cout << *max_element(dp.begin(), dp.end()) << endl;
  return 0;
}
