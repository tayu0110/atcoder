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
  cout << fixed << setprecision(20);
  ll n;
  cin >> n;
  ll len = n * (n+1) / 2;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  ll l = 0, r = *max_element(a.begin(), a.end())+1;
  while(r-l > 1) {
    ll m = (r+l) / 2;
    // DEBUG(l);DEBUG(r);DEBUG(m);
    vector<int> b(n+1);
    b[0] = 2*n;
    for(int i=0;i<n;i++) {
      b[i+1] += b[i];
      b[i+1] += a[i] < m ? -1 : 1;
    }
    FenwickTree<ll> ft(*max_element(b.begin(), b.end())+1);
    ll k = 0;
    for(int i=0;i<n;i++) {
      ft.add(b[i], 1);
      k += ft.getSum(0, b[i+1]+1);
    }
    // DEBUG_EN(k);
    if(k*2 >= len) l = m;
    else r = m;
  }
  cout << l << endl;
  return 0;
}
