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
const ld PI = acos(-1);
int modPow(long long a, long long n, long long p) {
  if (n == 0) return 1;
  a %= p;
  if (n == 1) return a;
  if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
  long long t = modPow(a, n / 2, p);
  return (t * t) % p;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll a, b, c;
  cin >> a >> b >> c;
  a %= 10;
  if(b == 1) {
    cout << a << endl;
    return 0;
  }
  if(a == 1 || a == 5 || a == 6) {
    cout << a << endl;
    return 0;
  }
  if(a == 4 || a == 9) {
    if(b % 2 == 0) {
      cout << a * a % 10 << endl;
    } else {
      cout << a << endl;
    }
    return 0;
  }
  int p = modPow(b, c, 4);
  ll t = 1;
  for(int i=0;i<p+4;i++) t *= a;
  cout << t % 10 << endl;
  return 0;
}
