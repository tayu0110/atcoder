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
int h, w;
vector<int> dx = {0, 1, 0, -1}, dy = {1, 0, -1, 0};
queue<pii> nt;
int solve(int s, int t) {
  int res = 0;
  int mx = h*w;
  nt.push({s, 1<<s});
  while(!nt.empty()) {
    auto [now, reached] = nt.front();
    nt.pop();
    if(now == s) {
      int r = 0;
      for(int i=0;i<h*w;i++) if(reached & 1<<i) r++;
      res = max(res, r);
      if(reached != 1<<s) continue;
    }
    int up = now-w;
    int down = now+w;
    if(up >= 0 && (1<<up & ~reached) && (1<<up & t)) {
      nt.push({up, reached | 1<<up});
    }
    if(down < mx && (1<<down && ~reached) && (1<<down & t)) {
      nt.push({down, reached | 1<<down});
    }
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> h >> w;
  int a = 0;
  string s;
  for(int i=0;i<h;i++) {
    cin >> s;
    for(int j=0;j<w;j++) {
      a <<= 1;
      if(s[j] == '#') a |= 1;
    }
  }
  int num = h*w;
  int ans = 0;
  for(int i=0;i<(1<<num);i++) {
    if((i & a) != 0) continue;
    int mx = 0;
    for(int j=0;j<num;j++) {
      if(1<<j & i) mx = max(mx, solve(j, i));
    }
    ans = max(mx, ans);
  }
  if(ans < 3) cout << -1 << endl;
  else cout << ans << endl;
  return 0;
}
