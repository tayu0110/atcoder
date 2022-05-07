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
int parse(string s) {
  int h = stoi(s.substr(0, 2)) * 60;
  int m = stoi(s.substr(3));
  int res = (h + m) % (24 * 60);
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    string s, t;
    cin >> s >> t;
    p[i] = {parse(s), parse(t)};
  }
  vector<int> dp(1<<n, inf);
  dp[0] = 0;
  for(int i=1;i<(1<<n);i++) {
    vector<pii> t;
    for(int j=0;j<n;j++) if(i & (1<<j)) {
      auto [l, r] = p[j];
      if(r < l) {
        t.push_back({l, 24*60-1});
        t.push_back({0, r});
      } else t.push_back(p[j]);
    }
    sort(t.begin(), t.end());
    int end = -1;
    bool f = true;
    for(auto [l, r] : t) {
      if(l < end) f = false;
      end = r;
    }
    if(f) dp[i] = 1;
    for(int j=i;j;--j&=i) {
      int k = i ^ j;
      dp[i] = min(dp[i], dp[j] + dp[k]);
    }
  }
  cout << dp[(1<<n)-1] << endl;

  return 0;
}
