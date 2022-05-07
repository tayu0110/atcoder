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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n, ga, sa, ba, gb, sb, bb;
  cin >> n >> ga >> sa >> ba >> gb >> sb >> bb;
  vector<ll> a = {ga, sa, ba};
  vector<ll> b = {gb, sb, bb};
  vector<ll> dp1(n+1, -1);
  dp1[0] = n;
  for(int i=0;i<3;i++) {
    for(int j=0;j<n+1;j++) {
      if(dp1[j] < 0) continue;
      if(j + a[i] > n) continue;
      dp1[j + a[i]] = max(dp1[j + a[i]], dp1[j] + b[i] - a[i]);
    }
  }
  int m = *max_element(dp1.begin(), dp1.end());
  DEBUG_EN(m);
  vector<ll> dp2(m+1, -1);
  dp2[0] = m;
  for(int i=0;i<3;i++) {
    for(int j=0;j<m+1;j++) {
      if(dp2[j] < 0) continue;
      if(j + b[i] > m) continue;
      dp2[j + b[i]] = max(dp2[j + b[i]], dp2[j] + a[i] - b[i]);
    }
  }
  ll l = *max_element(dp2.begin(), dp2.end());
  cout << l << endl;
  return 0;
}
