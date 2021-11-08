#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

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
bool check(ll a, ll b, ll c, ll x, ll y) {
  ll l, m, nx, ny;
  l = (a-1) / x + 1;
  ny = y - l;
  m = (b-1) / x + 1;
  ny -= m;
  if(ny * x >= c) {
    return true;
  }
  ny = y - l;
  if(ny > 0) {
    m = (b-1) / ny + 1;
    nx = x - m;
    if(ny * nx >= c) {
      return true;
    }
  }
  l = (a-1) / y + 1;
  if(nx > 0) {
    nx = x - l;
    m = (b-1) / nx + 1;
    ny = y - m;
    if(ny * nx >= c) {
      return true;
    }
  }
  nx = x - l;
  m = (b-1) / y + 1;
  nx -= m;
  if(y * nx >= c) {
    return true;
  }
  return false;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll x, y, a, b, c;
  cin >> x >> y >> a >> b >> c;
  if(x * y < a+b+c) {
    cout << "No" << endl;
    return 0;
  }
  vector<ll> p = {a, b, c};
  sort(p.begin(), p.end(), greater<ll>());
  a = p[0], b = p[1], c = p[2];
  bool f = check(a, b, c, x, y);
  f |= check(a, c, b, x, y);
  f |= check(b, c, a, x, y);
  f |= check(b, a, c, x, y);
  f |= check(c, a, b, x, y);
  f |= check(c, b, a, x, y);
  if(f) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
