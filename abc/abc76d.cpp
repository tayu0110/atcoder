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
  cin >> n;
  vector<int> t(n), v(n);
  for(int i=0;i<n;i++) cin >> t[i], t[i] *= 2;
  for(int i=0;i<n;i++) cin >> v[i];
  for(int i=1;i<n;i++) t[i] += t[i-1];
  vector<pair<ld, ld>> range(t[n-1]+1, {0, 0});
  {
    int l = 1;
    for(int i=0;i<n;i++) {
      if(l != 1) {
        range[l-1] = min(make_pair(v[i-1], v[i-1]), make_pair(v[i], v[i]));
      }
      while(l <= t[i]) {
        range[l] = {v[i], v[i]};
        l++;
      }
    }
  }
  range[range.size()-1] = {0, 0};
  for(int i=0;i<range.size()-1;i++) {
    if(range[i] < range[i+1]) {
      range[i+1].first = range[i].first + 0.5;
    } else if(range[i] == range[i+1]) {
      continue;
    } else {
      int l = i;
      while(l >= 0 && range[l].first > range[l+1].first+0.5) {
        range[l].first = range[l+1].first + 0.5;
        l--;
      }
    }
  }
  // for(int i=0;i<range.size();i++) {
  //   DEBUG(i);DEBUG_EN(range[i].first);
  // }
  ld ans = 0;
  for(int i=0;i<range.size();i++) {
    ans += range[i].first / 2;
  }
  cout << ans << endl;
  return 0;
}
