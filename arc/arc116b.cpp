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

using namespace std;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 998244353;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

struct SegmentTree {
  int sz;
  vector<int> t;
  SegmentTree(int n) {
    sz = 1;
    while(sz < n) sz *= 2;
    t.assign(2 * sz - 1, 0);
  }
  void update(int idx, int val) {
    idx += sz - 1;
    t[idx] = val;
    while(idx > 0) {
      idx = (idx - 1) / 2;
      if(t[idx] > val) break;
      t[idx] = val;
    }
    return;
  }
  int getMax(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return 0;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return 0;
    if(l <= a && r >= b) return t[now];
    int res = 0;
    res = max(res, getMax(l, r, 2*now+1, a, (a+b)/2));
    res = max(res, getMax(l, r, 2*now+2, (a+b)/2, b));
    return res;
  }
};

ll modPow(long long a, long long n, long long p) {
  if (n == 0) return 1;
  if (n == 1) return a % p;
  if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
  long long t = modPow(a, n / 2, p);
  return (t * t) % p;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) {
    cin >> a[i];
  }
  sort(a.begin(), a.end());
  ll sum = 0;
  ll bin = 1;
  for(int i=n-2;i>=0;i--) {
    sum += a[i] * bin % MOD;
    sum %= MOD;
    bin *= 2;
    bin %= MOD;
  }
  ll ans = 0;
  for(int i=n-1;i>=0;i--) {
    ans += a[i] * (a[i] + sum) % MOD;
    ans %= MOD;
    if(i-1 >= 0) {
      sum -= a[i-1];
      while(sum < 0 || sum % 2 == 1) sum += MOD;
      sum /= 2;
      sum + a[i-1];
      sum %= MOD;
    }
  }
  cout << ans << endl;
  // ll ans = 0;
  // for(int i=n-1;i>=0;i--) {
  //   ll bin = 1;
  //   ll sum = a[i];
  //   for(int j=i-1;j>=0;j--) {
  //     ll t = a[j] * bin;
  //     t %= MOD;
  //     sum += t;
  //     sum %= MOD;
  //     bin *= 2;
  //     bin %= MOD;
  //   }
  //   ll k = a[i] * sum % MOD;
  //   ans = ans + k % MOD;
  // }
  // cout << ans << endl;
  return 0;
}
