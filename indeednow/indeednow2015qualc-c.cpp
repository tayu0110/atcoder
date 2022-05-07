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
  int n;
  cin >> n;
  vector<vector<int>> t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--; b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  vector<int> res;
  vector<bool> ck(n, false);
  heap<int> nt;
  nt.push(0);
  while(!nt.empty()) {
    int now = nt.top();
    nt.pop();
    if(ck[now]) continue;
    ck[now] = true;
    res.push_back(now+1);
    for(auto to : t[now]) {
      if(ck[to]) continue;
      nt.push(to);
    }
  }
  for(int i=0;i<n;i++) {
    if(i) cout << " "; cout << res[i];
  }
  cout << endl;
  return 0;
}
