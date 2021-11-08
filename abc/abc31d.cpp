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
using pss = pair<string, string>;
vector<string> ans(10);
bool dfs(int now, vector<pss> &p) {
  if(!p[now].first.length()) {
    if(!p[now].second.length()) now++;
    else return false;
  }
  if(now == p.size()) return true;
  pss &tmp = p[now];
  string &s = tmp.first;
  string &t = tmp.second;
  int num = *(--s.end())-'0';
  s.pop_back();
  bool f = false;
  if(ans[num] != "") {
    string v;
    while(v.length() < 3 && t.length()) {
      v = *(--t.end()) + v;
      t.pop_back();
      if(v == ans[num]) {
        f = dfs(now, p);
        if(f) return f;
        else break;
      }
    }
    s.push_back(num+'0');
    t += v;
  } else {
    string v;
    while(v.length() < 3 && t.length()) {
      v = *(--t.end()) + v;
      t.pop_back();
      ans[num] = v;
      f = dfs(now, p);
      if(f) return f;
      ans[num].clear();
    }
    t += v;
    s.push_back(num+'0');
  }
  return f;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int k, n;
  cin >> k >> n;
  vector<pss> p;
  for(int i=0;i<n;i++) {
    string s, t;
    cin >> s >> t;
    if(s.length() == t.length() || s.length()*3 == t.length()) {
      int len = t.length() / s.length();
      for(int i=0,j=0;i<t.length();i+=len,j++) {
        int num = s[j] - '0';
        ans[num] = t.substr(i, len);
      }
    } else if(s.length() == 1) {
      int num = stoi(s);
      ans[num] = t;
    } else p.push_back({s, t});
  }
  n = p.size();
  if(n) {
    sort(p.begin(), p.end(), [&](pss l, pss r) {
      return l.second.length() < r.second.length();
    });
    dfs(0, p);
  }
  for(int i=1;i<k+1;i++) cout << ans[i] << endl;
  return 0;
}
