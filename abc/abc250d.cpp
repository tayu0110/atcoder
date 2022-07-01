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
  ll n;
  cin >> n;
  const int mx = 1000100;
  vector<ll> prime;
  vector<int> check(mx, -1);
  for(int i=2;i<mx;i++) {
    if(check[i] < 0) {
      prime.push_back(i);
      for(int j=1;i*j<mx;j++) check[i*j] = i;
    }
  }
  ll ans = 0;
  for(int i=0;i<prime.size();i++) {
    ll tri = prime[i] * prime[i] * prime[i];
    ll rem = n / tri;
    auto it = upper_bound(prime.begin(), prime.end(), rem);
    if(it == prime.begin()) continue;
    it--;
    ll t = *it;
    if(t < prime[i]) {
      ans += it - prime.begin() + 1;
    } else {
      ans += i;
    }
  }
  cout << ans << endl;
  return 0;
}
