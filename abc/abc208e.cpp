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
const ll INF = 1LL << 58;
const int inf = 1 << 29;
map<ll, ll> memo[20][2][2];
// ll rec(int now, bool is_less, bool hzero, ll k) {
//   if(now == s.length()) return 1LL;
//   if(memo[now][is_less][hzero].find(k) != memo[now][is_less][hzero].end()) return memo[now][is_less][hzero][k];
//   DEBUG(now);DEBUG_EN(k);
//   ll res = 0;
//   int t = s[now] - '0';
//   for(ll i=0;i<t;i++) {
//     if(i && k < i) continue;
//     bool f = !i && hzero;
//     ll nk = k == INF ? INF : (!i ? INF : k / i);
//     res += rec(now+1, false, f, nk);
//   }
//   ll mx = is_less ? 9 : t;
//   for(ll i=t;i<=mx;i++) {
//     if(i && k < i) continue;
//     if(!i) {
//       if(hzero) res += rec(now+1, is_less, true, k);
//       else res += rec(now+1, is_less, false, INF);
//     } else {
//       if(k == INF) res += rec(now+1, is_less, false, INF);
//       else res += rec(now+1, is_less, false, k / i);
//     }
//   }
//   return memo[now][is_less][hzero][k] = res;
// }
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  ll n, k;
  cin >> n >> k;
  string s = to_string(n);
  // cout << rec(0, false, true, k) << endl;
  memo[0][0][1][1] = 1;
  for(int i=0;i<s.length();i++) {
    for(int is_less=0;is_less<2;is_less++) {
      for(int hzero=0;hzero<2;hzero++) {
        map<ll, ll> &now = memo[i][is_less][hzero];
        ll t = s[i] - '0';
        for(auto [fi, se] : now) {
          for(ll j=0;j<10;j++) {
            if(!is_less && t < j) continue;
            int nis_less = is_less;
            if(j < t) nis_less = 1;
            int nhzero = hzero;
            if(0 < j) nhzero = 0;
            ll nfi = fi;
            if(!nhzero) {
              if(fi*j > k) nfi = INF;
              else nfi *= j;
            }
            memo[i+1][nis_less][nhzero][nfi] += se;
          }
        }
      }
    }
  }
  ll ans = 0;
  for(int i=0;i<2;i++) for(auto [fi, se] : memo[s.length()][i][0]) {
    if(fi <= k) ans += se;
  }
  cout << ans << endl;
  return 0;
}
