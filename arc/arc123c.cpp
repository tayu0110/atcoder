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
bool check(ll n, int mx, int now, int used) {
  if(mx == now) {
    if(n % 10) return false;
    now = 0;
    used = 1;
    n /= 10;
    if(!n) return true;
    for(int i=1;i<mx;i++) {
      bool f = check(n, i, now, 1);
      if(f) return f;
    }
  }
  for(int i=used;i<4;i++) {
    if(n - i < 0) continue;
    bool f = check(n-i, mx, now+1, i);
    if(f) return f;
  }
  return false;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  while(t--) {
    ll n;
    cin >> n;
    bool f = false;
    for(int i=1;i<5;i++) {
      if(check(n, i, 0, 1)) {
        cout << i << endl;
        f = true;
        break;
      }
    }
    if(!f) cout << 5 << endl;
  }
  return 0;
}
