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

const ll MOD = 3;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  string s;
  cin >> s;
  map<char, ll> mp;
  mp['B'] = 0;
  mp['W'] = 1;
  mp['R'] = 2;
  vector<int> f(n), g(n);
  g[0] = 1;
  for(int i=1;i<n;i++) {
    f[i] = f[i-1];
    g[i] = g[i-1];
    int tmp = i;
    while(tmp % 3 == 0) {
      f[i]++;
      tmp /= 3;
    }
    g[i] = g[i] * tmp % 3;
  }
  int ans = 0;
  for(int i=0;i<n;i++) {
    int now = mp[s[i]];
    if(f[n-1] - f[i] - f[n-1-i] == 0) {
      int s = g[n-1];
      int t = g[i] * g[n-1-i];
      int d;
      if(s == 1) {
        if(t == 1 || t == 4) d = 1;
        else if(t == 2) d = 2;
      } else {
        if(t == 1 || t == 4) d = 2;
        else if(t == 2) d = 1;
      }
      ans += d * now;
    }
  }
  if(n % 2 == 0) ans = (-ans % 3 + 12) % 3;
  else ans %= 3;
  if(ans == 0) cout << "B" << endl;
  else if(ans == 1) cout << "W" << endl;
  else cout << "R" << endl;
  return 0;
}
