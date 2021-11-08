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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll a, b, c;
  cin >> n >> a >> b >> c;
  vector<string> s(n);
  for(int i=0;i<n;i++) cin >> s[i];
  map<string, ll> mp;
  mp["A"] = a; mp["B"] = b; mp["C"] = c;
  vector<char> res;
  auto f = [&](string l, string r, string nt) {
    if(mp[l] < mp[r]) {
      mp[l]++; mp[r]--;
      res.push_back(l[0]);
    } else if(mp[l] > mp[r]) {
      mp[l]--; mp[r]++;
      res.push_back(r[0]);
    } else {
      string t = l + r;
      if(nt == "ABC" || t == nt) {
        mp[l]--; mp[r]++;
        res.push_back(r[0]);
      } else if(l[0] == nt[0] || l[0] == nt[1]) {
        mp[l]++; mp[r]--;
        res.push_back(l[0]);
      } else {
        mp[l]--; mp[r]++;
        res.push_back(r[0]);
      }
    }
  };
  for(int i=0;i<n;i++) {
    if(i != n-1) f(s[i].substr(0, 1), s[i].substr(1, 1), s[i+1]);
    else f(s[i].substr(0, 1), s[i].substr(1, 1), "ABC");
    if(mp["A"] < 0 || mp["B"] < 0 || mp["C"] < 0) {
      cout << "No" << endl;
      return 0;
    }
  }
  cout << "Yes" << endl;
  for(auto e : res) cout << e << endl;
  return 0;
}
