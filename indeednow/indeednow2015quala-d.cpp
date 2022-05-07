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
void solve2(int now, int pos, int d, int h, int w, vector<int> &s, map<vector<int>, int> &mp);
void solve(int now, int prev, int h, int w, vector<int> s, map<vector<int>, int> &mp) {
  if(mp.find(s) != mp.end()) mp[s] = min(mp[s], now);
  else mp[s] = now;
  if(now == 12) return;
  int pos = -1;
  for(int i=0;i<s.size();i++) if(s[i] == 0) {
    pos = i;
    break;
  }
  if(pos % w && pos-1 != prev) solve2(now, pos, -1, h, w, s, mp);
  if(pos % w != w-1 && pos+1 != prev) solve2(now, pos, 1, h, w, s, mp);
  if(pos - w >= 0 && pos-w != prev) solve2(now, pos, -w, h, w, s, mp);
  if(pos + w < s.size() && pos+w != prev) solve2(now, pos, w, h, w, s, mp);
}
void solve2(int now, int pos, int d, int h, int w, vector<int> &s, map<vector<int>, int> &mp) {
  swap(s[pos], s[pos+d]);
  solve(now+1, pos, h, w, s, mp);
  swap(s[pos], s[pos+d]);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<vector<int>> c(h, vector<int>(w));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> c[i][j];
  vector<int> s(h*w), t(h*w);
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) s[i * w + j] = c[i][j];
  for(int i=1;i<h*w+1;i++) t[i-1] = i % (h*w);
  map<vector<int>, int> mps, mpt;
  solve(0, -1, h, w, s, mps);
  solve(0, -1, h, w, t, mpt);
  int mn = 24;
  for(auto [f, s] : mps) {
    if(mpt.find(f) != mpt.end()) {
      mn = min(mn, s + mpt[f]);
    }
  }
  cout << mn << endl;
  return 0;
}
