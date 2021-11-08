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
#include<bitset>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

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

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<set<int>> prev(n);
  for(int i=0;i<m;i++) {
    int x, y;
    cin >> x >> y;
    x--;y--;
    prev[y].insert(x);
  }
  vector<set<int>> pl((1<<n));
  for(int i=0;i<(1<<n);i++) {
    set<int> ck;
    for(int j=0;j<n;j++) {
      if(i & (1<<j)) for(auto e : prev[j]) pl[i].insert(e);
    }
  }
  vector<vector<ll>> dp((1<<n), vector<ll>(n+1, 0));
  for(int i=0;i<n;i++) dp[(1<<i)][1] = 1;
  for(int i=1;i<n;i++) {
    for(int j=0;j<(1<<n);j++) {
      set<int> ck = pl[j];
      for(int l=0;l<n;l++) {
        if(j & (1<<l)) continue;
        if(ck.find(l) != ck.end()) continue;
        int t = j | (1<<l);
        dp[t][i+1] += dp[j][i];
      }
    }
  }
  cout << dp[(1<<n)-1][n] << endl;
  return 0;
}
