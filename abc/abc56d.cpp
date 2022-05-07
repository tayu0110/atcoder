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
  int n, k;
  cin >> n >> k;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  ll sum = accumulate(a.begin(), a.end(), 0LL);
  if(sum < k) {
    cout << n << endl;
    return 0;
  }
  sort(a.begin(), a.end(), greater<ll>());
  int ans = 0;
  map<ll, int> memo;
  for(int i=0;i<n;i++) {
    if(a[i] >= k) continue;
    if(sum - a[i] < k) continue;
    if(memo.find(a[i]) != memo.end()) {
      ans += memo[a[i]];
      continue;
    }
    set<ll> dp;
    dp.insert(0);
    bool f = false;
    for(int j=0;j<n;j++) {
      if(i == j) continue;
      set<ll> tmp;
      for(auto e : dp) {
        if(k-a[i] <= e && e < k) f = true;
        if(k-a[i] <= e+a[j] && e+a[j] < k) f = true;
        if(f) break;
        if(e < k) tmp.insert(e);
        if(e + a[j] < k) tmp.insert(e + a[j]);
      }
      if(f) break;
      dp.swap(tmp);
    }
    if(!f) {
      memo[a[i]]++;
      ans++;
    } else memo[a[i]] = 0;
  }
  cout << ans << endl;
  return 0;
}
