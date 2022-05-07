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
struct heap {
  priority_queue<T, vector<T>, greater<T>> pq;
  heap() : pq() {}
  heap(priority_queue<T, vector<T>, greater<T>> pq) : pq(pq) {}
  void push(T c) { pq.push(c); }
  T top() { return pq.top(); }
  void pop() { pq.pop(); }
  bool empty() { return pq.empty(); }
  int size() { return pq.size(); }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  if(k == 1) {
    cout << accumulate(a.begin(), a.end(), 0LL) << endl;
    return 0;
  }
  ll l = 0, r = accumulate(a.begin(), a.end(), 0LL) / k + 1;
  sort(a.begin(), a.end());
  while(r-l > 1) {
    ll m = (r+l) / 2;
    ll now = k;
    auto it = lower_bound(a.begin(), a.end(), m);
    now -= a.end() - it;
    int pos = it - a.begin();
    ll sum = 0;
    for(int i=0;i<pos;i++) sum += a[i];
    if(sum >= m*now) l = m;
    else r = m;
  }
  cout << l << endl;
  return 0;
}
