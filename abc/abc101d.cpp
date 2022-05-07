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
  ll k;
  cin >> k;
  if(k < 10) {
    for(int i=1;i<=k;i++) cout << i << endl;
    return 0;
  }
  deque<pair<ll, ld>> st;
  for(ll i=1;i<=9;i++) st.push_back({i, 1});
  ll t = 10;
  ll now = 1;
  while(st.size() < k + 150) {
    ll s = now * t + (t-1);
    ll r = 0;
    ll p = s;
    while(p) {
      r += p % 10;
      p /= 10;
    }
    ld d = (ld)s / r;
    bool f = false;
    while(!st.empty()) {
      auto [j, mn] = st.back();
      if(s < j) break;
      if(mn > d) {
        st.pop_back();
        f = true;
      } else break;
    }
    DEBUG(s);DEBUG_EN(f);
    if(!f) {
      if(st.empty()) st.push_back({s, d});
      else {
        auto [j, mn] = st.back();
        if(j < s) st.push_back({s, d});
      }
    } else {
      now = 1;
      t *= 10;
      continue;
    }
    now++;
  }

  // for(ll i=1;i<=10000000;i++) {
  //   int now = i;
  //   int s = 0;
  //   while(now) {
  //     s += now % 10;
  //     now /= 10;
  //   }
  //   if(st.empty()) st.push_back({i, (ld)i/s});
  //   else {
  //     while(!st.empty()) {
  //       auto [j, mn] = st.back();
  //       if(mn > (ld)i/s) st.pop_back();
  //       else break;
  //     }
  //     st.push_back({i, (ld)i/s});
  //   }
  // }
  for(int i=0;i<k;i++) {
    auto [j, _] = st.front();
    st.pop_front();
    cout << j << endl;
  }
  return 0;
}
