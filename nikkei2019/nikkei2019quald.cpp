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
  vector<vector<int>> t(n), rev(n);
  vector<int> out(n);
  for(int i=0;i<n+m-1;i++) {
    int u, v;
    cin >> u >> v;
    u--; v--;
    t[u].push_back(v);
    rev[v].push_back(u);
    out[u]++;
  }
  queue<int> nt;
  vector<int> rank(n, 0);
  for(int i=0;i<n;i++) {
    if(!out[i]) nt.push(i);
  }
  vector<bool> ck(n, false);
  while(!nt.empty()) {
    int now = nt.front(); nt.pop();
    if(ck[now]) continue;
    ck[now] = true;
    for(auto from : rev[now]) {
      out[from]--;
      rank[from] = max(rank[from], rank[now]+1);
      if(!out[from]) nt.push(from);
    }
  }
  vector<int> ans(n, -1);
  int root = max_element(rank.begin(), rank.end()) - rank.begin();
  nt.push(root);
  ans[root] = 0;
  while(!nt.empty()) {
    int now = nt.front(); nt.pop();
    for(auto to : t[now]) {
      if(rank[now] - rank[to] == 1) {
        nt.push(to);
        ans[to] = now+1;
      }
    }
  }
  for(auto e : ans) cout << e << endl;
  return 0;
}
