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
  int n, k;
  cin >> n >> k;
  vector<int> p(n);
  for(int i=0;i<n;i++) cin >> p[i], p[i]--;
  DEBUG_EN("reached");
  FenwickTree<int> ft(n);
  for(int i=0;i<k-1;i++) ft.add(p[i], 1);
  for(int i=k-1;i<n;i++) {
    ft.add(p[i], 1);
    int pos = ft.lower_bound(i+2-k);
    cout << pos+1 << endl;
  }
  return 0;
}
