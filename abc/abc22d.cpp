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
  vector<pair<ld, ld>> a(n), b(n);
  pair<ld, ld> ag, bg;
  auto f1 = [&](vector<pair<ld, ld>> &t, pair<ld, ld> &p) {
    for(int i=0;i<n;i++) {
      ld x, y;
      cin >> x >> y;
      t[i] = {x, y};
      p.first += x;
      p.second += y;
    }
  };
  f1(a, ag); f1(b, bg);
  auto f2 = [&](ld &mx, pair<ld, ld> &p, vector<pair<ld, ld>> &t) {
    for(int i=0;i<n;i++) {
      auto [x, y] = t[i];
      ld nx = (n * x - p.first) / n;
      ld ny = (n * y - p.second) / n;
      ld d = sqrtl(nx * nx + ny * ny);
      mx = max(d, mx);
    }
  };
  ld mxa = 0, mxb = 0;
  f2(mxa, ag, a); f2(mxb, bg, b);
  cout << mxb / mxa << endl;
  return 0;
}
