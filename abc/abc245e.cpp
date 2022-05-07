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
  int n, m;
  cin >> n >> m;
  vector<pll> s(n), t(m);
  for(int i=0;i<n;i++) cin >> s[i].first;
  for(int i=0;i<n;i++) cin >> s[i].second;
  for(int i=0;i<m;i++) cin >> t[i].first;
  for(int i=0;i<m;i++) cin >> t[i].second;
  sort(s.begin(), s.end());
  sort(t.begin(), t.end());
  multiset<ll> ck;
  for(int i=n-1;i>=0;i--) {
    auto [a, b] = s[i];
    while(t.size() && t.back().first >= a) {
      auto [c, d] = t.back();
      ck.insert(d);
      t.pop_back();
    }
    auto it = ck.lower_bound(b);
    if(it == ck.end()) {
      cout << "No" << endl;
      return 0;
    }
    ck.erase(it);
  }
  cout << "Yes" << endl;
  return 0;
}
