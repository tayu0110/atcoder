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
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
int solve(int s, vector<ll> &a, vector<ll> &b) {
  int n = a.size();
  int res = 0;
  for(int i=s;i<n+s;i++) {
    if(b[i%n] != gcd(a[i%n], a[(i+1)%n])) {
      res++;
      i += 2;
    }
  }
  return res;
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> b(n);
  for(int i=0;i<n;i++) cin >> b[i];
  vector<ll> a(n);
  for(int i=0;i<n;i++) a[i] = lcm(b[(i-1+n)%n], b[i]);
  int ans = inf;
  for(int i=0;i<3;i++) ans = min(ans, solve(i, a, b));
  cout << ans << endl;
  return 0;
}
