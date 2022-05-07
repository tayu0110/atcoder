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
  const int m = (1<<n);
  vector<ll> c(m-1);
  for(int i=0;i<m-1;i++) cin >> c[i];
  vector<pll> p(m);
  for(int i=1;i<m;i++) p[i] = {c[i-1], i};
  sort(p.begin(), p.end());
  ll ans = 0;
  set<ll> ck;
  ck.insert(0);
  for(int i=1;i<m;i++) {
    auto [v, idx] = p[i];
    if(ck.find(idx) != ck.end()) continue;
    set<ll> tmp;
    tmp.insert(0);
    for(auto e : ck) {
      tmp.insert(e ^ idx);
      tmp.insert(e);
    }
    ans += v;
    ck.swap(tmp);
  }
  cout << ans << endl;
  return 0;
}
