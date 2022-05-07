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
  int n;
  cin >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i], a[i]--;
  int q;
  cin >> q;
  map<int, vector<int>> mp;
  for(int i=0;i<n;i++) mp[a[i]].push_back(i);
  for(int i=0;i<n;i++) {
    if(mp.find(i) == mp.end()) continue;
    if(mp[i].size() == 1) mp.erase(i);
  }
  vector<pair<pii, int>> p(q);
  for(int i=0;i<q;i++) {
    int l, r;
    cin >> l >> r;
    p[i] = {{l-1, r}, i};
  }
  sort(p.begin(), p.end());
  vector<vector<pii>> t(n);
  for(int i=0;i<q;i++) {
    auto [f, s] = p[i];
    t[f.first].push_back({s, i});
  }
  for(int i=0;i<n;i++) {
    if(!t[i].empty()) {
      for(auto r : t[i]) {
        for(auto [f, s] : mp) {
          int span = lower_bound(s.begin(), s.end(), r) - lower_bound(s.begin(), s.end(), i);

        }
      }
    }
    int k = lower_bound(mp[a[i]].begin(), mp[a[i]].end(), i) - mp[a[i]].begin();
    if(mp[a[i]].size() - k < 2) mp.erase(a[i]);
  }
  return 0;
}
