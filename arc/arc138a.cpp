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

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  map<ll, int> mp;
  for(int i=0;i<n;i++) mp[a[i]] = 1;
  int cnt = 0;
  for(auto &[f, s] : mp) s = cnt++;
  vector<int> v(cnt, inf);
  for(int i=k;i<n;i++) v[mp[a[i]]] = min(v[mp[a[i]]], i);
  for(int i=cnt-2;i>=0;i--) v[i] = min(v[i+1], v[i]);
  int ans = inf;
  for(int i=0;i<k;i++) {
    int p = mp[a[i]]+1;
    if(p == cnt) continue;
    int t = v[p];
    if(t == inf) continue;
    ans = min(ans, t - i);
  }
  // print_with_space(v);
  if(ans == inf) cout << -1 << endl;
  else cout << ans << endl;
  return 0;
}
