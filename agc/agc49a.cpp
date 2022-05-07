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
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<string> s(n);
  for(int i=0;i<n;i++) cin >> s[i];
  Graph t(n);
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      if(s[i][j] == '1') t[i].push_back(j);
    }
  }
  vector<int> v(n);
  queue<int> nt;
  for(int i=0;i<n;i++) {
    vector<bool> reachable(n, false);
    nt.push(i);
    while(!nt.empty()) {
      int now = nt.front();
      nt.pop();
      if(reachable[now]) continue;
      reachable[now] = true;
      for(auto to : t[now]) nt.push(to);
    }
    for(int j=0;j<n;j++) if(reachable[j]) v[j]++;
  }
  double ans = 0;
  for(int i=0;i<n;i++) ans += (double)1 / v[i];
  cout << ans << endl;
  return 0;
}
