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
template<class T>
struct heap {
  priority_queue<T, vector<T>, greater<T>> pq;
  heap() : pq() {}
  heap(priority_queue<T, vector<T>, greater<T>> pq) : pq(pq) {}
  void push(T c) { pq.push(c); }
  T top() { return pq.top(); }
  void pop() { pq.pop(); }
  bool empty() { return pq.empty(); }
  int size() { return pq.size(); }
  void swap(heap<T> nt) { pq.swap(nt.pq); }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k, q;
  cin >> n >> k >> q;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  ll ans = INF;
  for(int i=0;i<n;i++) {
    ll t = a[i];
    heap<pll> nt;
    heap<ll> p;
    vector<int> s;
    for(int j=0;j<n;j++) {
      if(a[j] < t) {
        int sz = p.size();
        if(sz >= k) {
          while(!p.empty()) {
            nt.push({p.top(), s.size()});
            p.pop();
          }
        }
        p = heap<ll>();
        s.push_back(sz);
      } else {
        p.push(a[j]);
      }
    }
    if(p.size() >= k) {
      int sz = p.size();
      while(!p.empty()) {
        nt.push({p.top(), s.size()}); p.pop();
      }
      s.push_back(sz);
    }
    bool f = true;
    ll mn = INF, mx = -1;
    for(int j=0;j<q;j++) {
      f = false;
      while(!nt.empty()) {
        auto [x, idx] = nt.top(); nt.pop();
        if(s[idx] < k) continue;
        mn = min(mn, x); mx = max(mx, x);
        s[idx]--;
        f = true;
        break;
      }
    }
    if(!f) continue;
    ans = min(ans, mx-mn);
    // DEBUG(i);DEBUG(t);
    // DEBUG(ans);DEBUG(mx);DEBUG_EN(mn);
  }
  cout << ans << endl;
  return 0;
}
