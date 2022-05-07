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
// template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }
template<class T> void print_with_space(T p) { for(auto e : p) cerr << "(" << e.first << ", " << e.second << ") "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const double PI = acos(-1);
const double EPS = 1e-10;

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n;
  cin >> n;
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    int x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  ll a = n * (n-1) * (n-2) / 6, b = 0, c = 0;
  for(int i=0;i<n;i++) {
    auto [x, y] = p[i];
    vector<double> t;
    for(int j=0;j<n;j++) {
      if(i == j) continue;
      auto [nx, ny] = p[j];
      double arg = atan2(ny-y, nx-x);
      t.push_back(arg);
      t.push_back(arg + PI * 2);
    }
    sort(t.begin(), t.end());
    for(int j=0;j<n-1;j++) {
      b += upper_bound(t.begin(), t.end(), t[j] + PI / 2 + EPS) - lower_bound(t.begin(), t.end(), t[j] + PI / 2 - EPS);
      c += lower_bound(t.begin(), t.end(), t[j] + PI - EPS) - upper_bound(t.begin(), t.end(), t[j] + PI / 2 + EPS);
    }
  }
  a -= b + c;
  cout << a << " " << b << " " << c << endl;
  return 0;
}
