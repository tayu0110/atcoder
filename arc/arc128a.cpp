#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

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
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<pll> b;
  for(int i=0;i<n;i++) {
    if(b.empty()) b.push_back({a[i], i});
    else {
      if(b.back().first == a[i]) continue;
      else b.push_back({a[i], i});
    }
  }
  int m = b.size();
  vector<pll> res(m);
  for(int i=0;i<m;i++) {
    ll now = b[i].first;
    int idx = b[i].second;
    if(i == 0) {
      ll nt = b[i+1].first;
      res[i] = {(now > nt), idx};
    } else if(i == m-1) {
      ll prev = b[i-1].first;
      res[i] = {(now < prev), idx};
    } else {
      ll nt = b[i+1].first;
      ll prev = b[i-1].first;
      if(prev < now && now > nt) res[i] = {1, idx};
      else if(prev > now && now < nt) res[i] = {1, idx};
      else res[i] = {0, idx};
    }
  }
  vector<int> ans(n, 0);
  int cnt = 0;
  for(int i=0;i<m;i++) {
    int r = res[i].first;
    int idx = res[i].second;
    ans[idx] = r;
    cnt += r;
  }
  if(cnt % 2 == 1) {
    for(int i=n-1;i>=0;i--) {
      if(ans[i]) {
        ans[i] = 0;
        break;
      }
    }
  }
  for(int i=0;i<n;i++) {
    cout << ans[i];
    if(i == n-1) cout << endl;
    else cout << " ";
  }
  return 0;
}
