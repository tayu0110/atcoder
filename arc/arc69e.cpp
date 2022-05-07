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
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<pll> p(n);
  for(int i=0;i<n;i++) p[i] = {a[i], i};
  sort(p.begin(), p.end(), greater<pll>());
  p.push_back({0, -1});
  vector<ll> ans(n);
  ll prev = INF;
  ll pf = -1;
  ll sum = 0;
  for(int i=0;i<n+1;i++) {
    auto [f, s] = p[i];
    if(f < pf) {
      ans[prev] += sum - f * i;
      sum = f * i;
    }
    pf = f;
    prev = min(prev, s);
    sum += f;
    // DEBUG(f);DEBUG(s);
    // DEBUG(prev);DEBUG(pf);DEBUG_EN(sum);
  }
  for(auto e : ans) cout << e << endl;
  return 0;
}
