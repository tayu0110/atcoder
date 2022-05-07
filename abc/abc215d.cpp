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
  int n, m;
  cin >> n >> m;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  int mx = *max_element(a.begin(), a.end());
  vector<int> p(max(mx+1, m+1), -1);
  for(int i=2;i<p.size();i++) {
    if(p[i] > 0) continue;
    for(int j=1;i*j<p.size();j++) {
      if(p[i*j] > 0) continue;
      p[i*j] = i;
    }
  }
  set<int> ck;
  for(int i=0;i<n;i++) {
    int t = a[i];
    while(p[t] > 0) {
      ck.insert(p[t]);
      t /= p[t];
    }
  }
  set<int> ans;
  for(int i=2;i<m+1;i++) {
    int t = i;
    bool f = true;
    while(p[t] > 0) {
      if(ck.find(p[t]) != ck.end()) {
        f = false;
        break;
      }
      t /= p[t];
    }
    if(f) ans.insert(i);
  }
  ans.insert(1);
  cout << ans.size() << endl;
  for(auto e : ans) cout << e << endl;
  return 0;
}
