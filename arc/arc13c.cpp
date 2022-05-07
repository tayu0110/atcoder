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
  int n;
  cin >> n;
  ll ans = 0;
  for(int i=0;i<n;i++) {
    vector<ll> t(3);
    for(int j=0;j<3;j++) cin >> t[j];
    int m;
    cin >> m;
    vector<ll> mn = {INF, INF, INF};
    vector<ll> mx = {-INF, -INF, -INF};
    for(int j=0;j<m;j++) for(int k=0;k<3;k++) {
      ll a;
      cin >> a;
      mn[k] = min(mn[k], a);
      mx[k] = max(mx[k], a);
    }
    for(int j=0;j<3;j++) {
      ans ^= (mn[j]);
      ans ^= (t[j] - mx[j] - 1);
    }
  }
  cout << (ans ? "WIN" : "LOSE") << endl;
  return 0;
}
