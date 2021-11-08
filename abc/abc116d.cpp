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
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
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
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<pll> p(n);
  for(int i=0;i<n;i++) cin >> p[i].second >> p[i].first;
  sort(p.begin(), p.end(), greater<pll>());
  map<ll, ll> mp;
  ll sum = 0;
  heap<pll> nt;
  for(int i=0;i<k;i++) {
    int t = p[i].second;
    ll d = p[i].first;
    mp[t]++;
    nt.push(p[i]);
    sum += d;
  }
  sum += (ll)mp.size() * (ll)mp.size();
  vector<ll> res = {sum};
  for(int i=k;i<n;i++) {
    int t = p[i].second;
    ll d = p[i].first;
    if(mp.find(t) != mp.end()) continue;
    ll r = res[res.size()-1];
    while(!nt.empty()) {
      auto l = nt.top();
      nt.pop();
      if(mp[l.second] == 1) continue;
      ll sz = mp.size();
      r = r - l.first + d - sz * sz + (sz+1) * (sz+1);
      mp[l.second]--;
      mp[t]++;
      break;
    }
    res.push_back(r);
    if(nt.empty()) break;
  }
  cout << *max_element(res.begin(), res.end()) << endl;
  return 0;
}
