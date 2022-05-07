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

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

int n;
vector<int> h;
vector<tuple<int, int, int>> p;
vector<vector<short>> cnt;
vector<vector<int>> etime;
vector<ll> memo;
vector<bool> ck;
ll solve(int now) {
  if(now == n) return 0;
  if(ck[now]) return memo[now];
  ck[now] = true;
  ll res = 0;
  auto [e, s, m] = p[now];
  cnt[now].assign(n, -1);
  etime[now].assign(n, -1);
  for(int i=now+1;i<n;i++) {
    auto [ei, si, mi] = p[i];
    if(si < e) continue;
    if(si < etime[now][mi]) continue;
    etime[now][mi] = ei;
    cnt[now][mi]++;
    res = max(res, solve(i) + h[cnt[now][mi]]);
  }
  return memo[now] = res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n;
  h.resize(n);
  for(int i=0;i<n;i++) cin >> h[i];
  for(int i=0;i<n-1;i++) h[i+1] += h[i];
  n++;
  p.assign(n, {-1, -1, 0});
  for(int i=1;i<n;i++) {
    int m, s, e;
    cin >> m >> s >> e;
    p[i] = {e, s, m};
  }
  sort(p.begin(), p.end());
  ck.assign(n, false);
  cnt.assign(n, vector<short>(n, -1));
  etime.assign(n, vector<int>(n, -1));
  memo.resize(n);
  cout << solve(0) << endl;
  return 0;
}
