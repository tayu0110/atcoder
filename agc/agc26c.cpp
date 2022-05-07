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
void solve(string s, map<pair<string, string>, ll> &mp) {
  int n = s.length();
  for(int i=0;i<(1<<n);i++) {
    string t;
    string u;
    for(int j=0;j<n;j++) {
      if(i & (1<<j)) t += s[j];
      else u += s[j];
    }
    mp[{t, u}]++;
    mp[{u, t}]++;
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  string s;
  cin >> n >> s;
  string t = s.substr(n);
  reverse(t.begin(), t.end());
  s = s.substr(0, n);
  map<pair<string, string>, ll> mps, mpt;
  solve(s, mps);
  solve(t, mpt);
  ll ans = 0;
  for(auto [f, s] : mps) {
    auto [ff, fs] = f;
    if(mpt.count({fs, ff})) {
      ans += s * mpt[{fs, ff}];
    }
  }
  cout << ans / 4 << endl;
  return 0;
}
