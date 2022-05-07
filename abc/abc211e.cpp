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
int n, k;
ll t = 0;
ll now = 0;
vector<int> dx = {1, 0, -1, 0};
vector<int> dy = {0, 1, 0, -1};
set<ll> used;
set<ll> success;
int conv(int r, int c) { return r * n + c; }
bool check(int r, int c) { return r >= 0 && c >= 0 && r < n && c < n; }
int dfs(int rem) {
  ll mp = t | now;
  if(used.find(mp) != used.end()) return 0;
  used.insert(mp);
  if(!rem) {
    success.insert(now);
    return 1;
  }
  int res = 0;
  vector<ll> nt;
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      ll tmp = 1LL << conv(i, j);
      if(mp & tmp) continue;
      for(int l=0;l<4;l++) {
        int x = j + dx[l];
        int y = i + dy[l];
        if(!check(y, x)) continue;
        if(now & (1LL << conv(y, x))) {
          nt.push_back(tmp);
        }
      }
    }
  }
  for(auto e : nt) {
    now |= e;
    res += dfs(rem-1);
    now ^= e;
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n >> k;
  for(int i=0;i<n;i++) {
    string s;
    cin >> s;
    for(int j=0;j<n;j++) {
      if(s[j] == '#') t |= 1LL << conv(i, j);
    }
  }
  int res = 0;
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    ll tmp = 1LL << conv(i, j);
    if(t & tmp) continue;
    now |= tmp;
    res += dfs(k-1);
    now ^= tmp;
  }
  cout << res << endl;
  return 0;
}
