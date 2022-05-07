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
  int t;
  cin >> t;
  const int mx = 1000000000;
  while(t--) {
    int n;
    cin >> n;
    vector<pll> p(n);
    for(int i=0;i<n;i++) {
      ll l, r;
      cin >> l >> r;
      p[i] = {l, r};
    }
    sort(p.begin(), p.end());
    heap<ll> nt;
    int now = 1;
    bool f = true;
    int unused = 0;
    while(now <= mx) {
      while(unused < n && p[unused].first == now) {
        nt.push(p[unused].second);
        unused++;
      }
      if(nt.empty()) {
        if(unused == n) break;
        now = p[unused].first;
        continue;
      }
      int r = nt.top();
      nt.pop();
      if(r < now) {
        f = false;
        break;
      }
      now++;
    }
    if(f && unused == n && nt.empty()) cout << "Yes" << endl;
    else cout << "No" << endl;
  }
  return 0;
}
