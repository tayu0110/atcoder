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
  int q;
  cin >> q;
  vector<ll> ans;
  while(q--) {
    ll a, b;
    cin >> a >> b;
    ll t = a * b;
    if(a > b) swap(a, b);
    ll l = 0, r = b+1;
    while(r-l > 1) {
      ll m = (r+l) / 2;
      if(m * m <= t) l = m;
      else r = m;
    }
    ll res = (l-1) * 2;
    if(l * l != t) {
      if((t-1) / l == l) {
        if(l != a && l != b) res++;
      } else {
        r = (t-1) / l;
        if(l != a && r != b) res++;
        if(r != a && l != b) res++;
      }
    }
    if(l > a) res--;
    if(l > b) res--;
    ans.push_back(res);
  }
  for(auto e : ans) cout << e << endl;
  return 0;
}
