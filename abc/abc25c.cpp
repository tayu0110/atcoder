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
int rec(int now, vector<vector<int>> &t, vector<vector<int>> &b, vector<vector<int>> &c) {
  if(now == 9) {
    int score = 0;
    for(int i=0;i<3;i++) for(int j=0;j<3;j++) {
      if(i+1 < 3) {
        if(t[i][j] == t[i+1][j]) score += b[i][j];
      }
      if(j+1 < 3) {
        if(t[i][j] == t[i][j+1]) score += c[i][j];
      }
    }
    return score;
  }
  int score;
  if(now % 2 == 0) score = 0;
  else score = inf;
  for(int i=0;i<3;i++) for(int j=0;j<3;j++) {
    if(t[i][j] != 0) continue;
    if(now % 2 == 0) {
      t[i][j] = 1;
      score = max(rec(now+1, t, b, c), score);
      t[i][j] = 0;
    } else {
      t[i][j] = -1;
      score = min(rec(now+1, t, b, c), score);
      t[i][j] = 0;
    }
  }
  return score;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  const int h = 3, w = 3;
  vector<vector<int>> b(h-1, vector<int>(w)), c(h, vector<int>(w-1));
  int sum = 0;
  for(int i=0;i<h-1;i++) for(int j=0;j<w;j++) cin >> b[i][j], sum += b[i][j];
  for(int i=0;i<h;i++) for(int j=0;j<w-1;j++) cin >> c[i][j], sum += c[i][j];
  vector<vector<int>> t(h, vector<int>(w));
  int ans = rec(0, t, b, c);
  cout << ans << endl;
  cout << sum-ans << endl;
  return 0;
}
