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
  ll x;
  cin >> n >> x;
  vector<ll> t(n), a(n);
  for(int i=0;i<n;i++) cin >> t[i];
  for(int i=0;i<n;i++) cin >> a[i];
  if(accumulate(a.begin(), a.end(), 0LL) < x) {
    cout << -1 << endl;
    return 0;
  }
  const int maxt = 100100;
  vector<vector<ll>> p(maxt);
  for(int i=0;i<n;i++) p[t[i]].push_back(a[i]);
  int l = 0, r = maxt;
  while(r - l > 1) {
    int m = (r+l) / 2;
    priority_queue<ll> nt;
    for(int i=maxt-1;i>m;i--) {
      for(auto e : p[i]) nt.push(e);
    }
    ll sum = 0;
    for(int i=m;i>0;i--) {
      for(auto e : p[i]) nt.push(e);
      if(!nt.empty()) {
        sum += nt.top(); nt.pop();
      }
    }
    if(sum < x) l = m;
    else r = m;
  }
  if(r == maxt) cout << -1 << endl;
  else cout << r << endl;
  return 0;
}
