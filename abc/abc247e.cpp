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
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
int n, x, y;
ll solve(int l, int r, vector<int> &a) {
  int nl = l;
  int xp = -1, yp = -1;
  ll res = 0;
  for(ll i=l;i<r;i++) {
    if(a[i] == x) {
      if(yp != -1) {
        res += (yp-nl+1) * (r-i);
        xp = i;
        nl = yp+1;
        yp = -1;
      } else {
        xp = i;
      }
    } else if(a[i] == y) {
      if(xp != -1) {
        res += (xp-nl+1) * (r-i);
        yp = i;
        nl = xp+1;
        xp = -1;
      } else {
        yp = i;
      }
    }
  }
  return res;
}
ll solve2(vector<int> &a) {
  ll l = 0, r = 0;
  bool flag = false;
  ll res = 0;
  int cnt = 0;
  for(int i=0;i<n;i++) {
    if(a[i] != x) {
      flag = false;
      if(l >= 0 && r >= 0) {
        res += (r-l+1) * (r-l) / 2;
      }
      l = -1, r = -1;
    } else {
      cnt++;
      if(!flag) {
        l = r = i;
        flag = true;
      } else {
        r++;
      }
    }
  }
  if(l >= 0 && r >= 0) {
    res += (r-l+1) * (r-l) / 2;
  }
  return res+cnt;
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  cin >> n >> x >> y;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  if(x == y) {
    cout << solve2(a) << endl;
    return 0;
  }
  int l = 0, r = 0;
  while(l < n && (a[l] < y || x < a[l])) l++, r++;
  ll ans = 0;
  while(l < n) {
    while(r < n && y <= a[r] && a[r] <= x) r++;
    ans += solve(l, r, a);
    l = r;
    while(l < n && a[l] < y || x < a[l]) l++;
    r = l;
  }
  cout << ans << endl;
  return 0;
}
