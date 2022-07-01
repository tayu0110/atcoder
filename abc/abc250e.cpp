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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;

int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  set<int> ck;
  map<int, int> mp;
  int cnt = 0;
  for(int i=0;i<n;i++) {
    if(ck.find(a[i]) == ck.end()) {
      mp[a[i]] = cnt++;
      ck.insert(a[i]);
    }
    a[i] = mp[a[i]];
  }
  for(int i=0;i<n;i++) {
    if(mp.find(b[i]) == mp.end()) mp[b[i]] = cnt++;
    b[i] = mp[b[i]];
  }
  ck.clear();
  vector<int> pa(n);
  vector<pii> pb(n);
  for(int i=0;i<n;i++) {
    ck.insert(a[i]);
    pa[i] = ck.size();
  }
  ck.clear();
  for(int i=0;i<n;i++) {
    ck.insert(b[i]);
    auto it = ck.end();
    it--;
    pb[i] = {ck.size(), *it};
  }
  int q;
  cin >> q;
  while(q--) {
    int x, y;
    cin >> x >> y;
    x--; y--;
    int akind = pa[x];
    auto [bkind, bmx] = pb[y];
    if(akind == bkind && akind-1 == bmx) cout << "Yes" << endl;
    else cout << "No" << endl;
  }
  return 0;
}
