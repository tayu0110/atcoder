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
  ll h, w;
  int c, q;
  cin >> h >> w >> c >> q;
  vector<tuple<ll, ll, ll>> p(q);
  for(int i=0;i<q;i++) {
    int t;
    ll a, b;
    cin >> t >> a >> b;
    p[i] = {t, a, b};
  }
  vector<ll> v(c+1);
  set<ll> rck, cck;
  for(int i=q-1;i>=0;i--) {
    auto [t, a, b] = p[i];
    if(t == 1) {
      if(rck.find(a) != rck.end()) continue;
      v[b] += w;
      h--;
      rck.insert(a);
    } else {
      if(cck.find(a) != cck.end()) continue;
      v[b] += h;
      w--;
      cck.insert(a);
    }
  }
  for(int i=1;i<c+1;i++) {
    cout << v[i];
    if(i == c) cout << endl;
    else cout << " ";
  }
  return 0;
}
