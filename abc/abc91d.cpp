#pragma GCC target("avx")
#pragma GCC optimize("Ofast")
#pragma GCC optimize("unroll-loops")
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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
int n;
vector<int> a, b;
int solve(int d) {
  vector<int> s(n), t(n);
  int mask = 1 << (d+1);
  for(int i=0;i<n;i++) s[i] = a[i] % mask;
  for(int i=0;i<n;i++) t[i] = b[i] % mask;
  sort(t.begin(), t.end());
  int l = mask / 2, r = mask;
  int res = 0;
  for(int i=0;i<2;i++) {
    for(int j=0;j<n;j++) {
      auto lit = lower_bound(t.begin(), t.end(), l - s[j]);
      auto rit = lower_bound(t.begin(), t.end(), r - s[j]);
      res += rit - lit;
      res %= 2;
    }
    l *= 3;
    r *= 2;
  }
  return res;
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  cin >> n;
  a.resize(n);
  b.resize(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  int ans = 0;
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      ans ^= a[i] + b[j];
    }
  }
  cout << ans << endl;
  // int ans = 0;
  // for(int i=0;i<29;i++) ans |= solve(i) << i;
  // cout << ans << endl;
  return 0;
}
