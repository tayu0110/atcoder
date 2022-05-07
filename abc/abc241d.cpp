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
  int q;
  cin >> q;
  multiset<ll> ck;
  while(q--) {
    int idx;
    ll x;
    cin >> idx >> x;
    if(idx == 1) {
      ck.insert(x);
    } else if(idx == 2) {
      int k;
      cin >> k;
      auto it = ck.upper_bound(x);
      if(it == ck.begin()) {
        cout << -1 << endl;
        continue;
      }
      while(k && it != ck.begin()) it--, k--;
      if(!k) cout << *it << endl;
      else cout << -1 << endl;
    } else {
      int k;
      cin >> k;
      auto it = ck.lower_bound(x);
      while(it != ck.end() && --k) it++;
      if(!k && it != ck.end()) cout << *it << endl;
      else cout << -1 << endl;
    }
  }
  return 0;
}
