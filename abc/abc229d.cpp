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
  string s;
  int k;
  cin >> s >> k;
  int n = s.length();
  int ans = 0;
  {
    int now = 0;
    for(int i=0;i<n;i++) {
      if(s[i] == 'X') now++;
      else now = 0;
      ans = max(ans, now);
    }
    DEBUG_EN(ans);
  }
  if(k == 0) {
    cout << ans << endl;
    return 0;
  }
  vector<int> p(n+1);
  for(int i=1;i<n+1;i++) {
    if(s[i-1] == '.') p[i]++;
    p[i] += p[i-1];
  }
  int l = 0, r = 1;
  while(r < p.size()) {
    while(r+1 < p.size() && p[r+1] - p[l] <= k) r++;
    if(ans < r-l) {
      DEBUG(l);DEBUG_EN(r);
    }
    ans = max(ans, r-l);
    l++;
    if(l >= r) r = l+1;
  }
  cout << ans << endl;
  return 0;
}
