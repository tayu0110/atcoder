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
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  multiset<int> ck;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    int x, y;
    cin >> x >> y;
    p[i] = {y, x};
  }
  for(int i=0;i<m;i++) {
    int a;
    cin >> a;
    ck.insert(a);
  }
  sort(p.begin(), p.end());
  int ans = 0;
  for(int i=0;i<n;i++) {
    auto [y, x] = p[i];
    auto it = ck.lower_bound(x);
    if(it == ck.end()) continue;
    if(*it <= y) {
      ans++;
      ck.erase(it);
    }
  }
  cout << ans << endl;
  return 0;
}
