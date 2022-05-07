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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  sort(a.begin(), a.end(), greater<ll>());
  vector<int> bits(40, 0);
  for(int i=0;i<n;i++) {
    ll t = a[i];
    int cnt = 0;
    while(t) {
      bits[cnt] += (t & 1);
      t >>= 1;
      cnt++;
    }
  }
  ll sum = accumulate(a.begin(), a.end(), 0LL);
  ll ans = sum;
  for(int i=0;i<n;i++) {
    ll t = a[i];
    int cnt = 0;
    ll k = 1;
    ll tmp = sum;
    while(t) {
      if(t & 1) {
        tmp += (n - bits[cnt] * 2) * k;
      }
      t >>= 1;
      k <<= 1;
      cnt++;
    }
    ans = max(ans, tmp);
  }
  cout << ans << endl;
  return 0;
}
