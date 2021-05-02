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

// ll comb(ll n, ll k) {
//   ll t = k;
//   ll res = 1;
//   vector<ll> s;
//   set<ll> ck;
//   for(int i=1;i<=k;i++) ck.insert(i);
//   while(t--) {
//     ll l = n;
//     vector<ll> p;
//     for(auto e : ck) {
//       if(l % e == 0) {
//         l /= e;
//         p.push_back(e);
//       }
//     }
//     for(auto e : p) ck.erase(e);
//     s.push_back(l);
//     n--;
//   }
//   for(auto e : s) {
//     res *= e;
//     res %= MOD;
//   }
//   return res;
// }

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  // ll ans = 0;
  // vector<ll> memo(m+1, -1);
  // for(int i=1;i<=m;i++) {
  //   ll d = 0;
  //   for(int j=1;j*j<=i;j++) {
  //     if(i % j != 0) continue;
  //     if(i / j == j) d++;
  //     else d += 2;
  //   }
  //   ll k = n-1;
  //   ll l = d + k - 1;
  //   if(k > l - k) k = l - k;
  //   ll c;
  //   if(memo[d] != -1) c = memo[d];
  //   else {
  //     c = comb(l, k);
  //     c %= MOD;
  //     memo[d] = c;
  //   }
  //   ans += c;
  //   ans %= MOD;
  // }
  // cout << ans << endl;
  return 0;
}
