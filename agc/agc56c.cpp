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
template<class T>
class FenwickTree {
  vector<T> t;
  int sz;
public: 
  FenwickTree(int n) : sz(n+1), t(vector<T>(n+1, 0)) {}
  // fix the 0-based array to the 1-based array
  FenwickTree(vector<T> &s) : sz(s.size() + 1) {
    t.assign(sz, 0);
    for(int i=0;i<s.size();i++) add(i, s[i]);
  }
  // add(idx, val) => add val to idx th element
  // idx : 0-based index
  void add(int idx, T val) {
    idx++;
    while(idx < sz) {
      t[idx] += val;
      idx += idx & -idx;
    }
  }
  // getSum(l, r) => get the sum of [l, r)
  // l, r : 0-based index
  T getSum(int l, int r) {
    // need not fix l and r to 1-based index
    // because subGetSum() needs a 0-based index argument.
    return subGetSum(r) - subGetSum(l);
  }
  // lower_bound(val) => return 0-based index
  int lower_bound(T val) {
    if(val <= 0) return 0;
    int now = 0;
    int n = 1;
    while(n<<1 <= sz) n <<= 1;
    for(int i=n;i>0;i>>=1) {
      if(now+i < sz && t[now+i] < val) {
        val -= t[now+i];
        now += i;
      }
    }
    return now;
  }
  // upper_bound(val) => return 0-based index
  int upper_bound(T val) {
    if(val <= 0) return 0;
    int now = 0;
    int n = 1;
    while(n<<1 <= sz) n <<= 1;
    for(int i=n;i>0;i>>=1) {
      if(now+i < sz && t[now+i] <= val) {
        val -= t[now+i];
        now += i;
      }
    }
    return now;
  }
private:
  // subGetSum(r) => get the sum of [0, r)
  // r : 0-based index
  T subGetSum(int r) {
    if(r < 0) return 0;
    if(r >= sz) r = sz-1;
    T res = 0;
    while(r > 0) {
      res += t[r];
      r -= r & -r;
    }
    return res;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<pii> p(m);
  for(int i=0;i<m;i++) {
    int l, r;
    cin >> l >> r;
    l--;
    p[i] = {l, r};
  }
  sort(p.begin(), p.end());
  FenwickTree<int> ft(n);
  
  // string ans;
  // FenwickTree<int> ft(n);
  // priority_queue<int> zp;
  // for(int i=0;i<m;i++) {
  //   auto [l, r] = p[i];
  //   while(ans.length() < l) {
  //     zp.push(ans.length());
  //     ft.add(ans.length(), 1);
  //     ans += "0";
  //   }
  //   while(ans.length() < r) {
  //     if(ft.getSum(l, r) < (r-l) / 2) {
  //       zp.push(ans.length());
  //       ft.add(ans.length(), 1);
  //       ans += "0";
  //     } else {
  //       ans += "1";
  //     }
  //   }
  //   int now = r;
  //   queue<int> nt;
  //   while(ft.getSum(l, r) > (r-l) / 2) {
  //     now--;
  //     if(ans[now] != '1') continue;
  //     int op = zp.top();
  //     zp.pop();
  //     swap(ans[now], ans[op]);
  //     ft.add(op, -1);
  //     ft.add(now, 1);
  //     nt.push(op);
  //   }
  //   while(!nt.empty()) {
  //     zp.push(nt.front()); nt.pop();
  //   }
  // }
  // while(ans.length() < n) ans += "0";
  // for(int i=0;i<m;i++) {
  //   auto [l, r] = p[i];
  //   DEBUG(l);DEBUG(r);DEBUG_EN(ft.getSum(l, r));
  // }
  // DEBUG_EN(ans.length());
  // cout << ans << endl;
  return 0;
}
