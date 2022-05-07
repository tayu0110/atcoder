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
using weightedGraph = vector<vector<Edge>>;
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<vector<int>> res(t);
  for(int idx=0;idx<t;idx++) {
    int n;
    cin >> n;
    map<pii, int> mp;
    Graph t(n);
    bool f = true;
    for(int i=0;i<n-1;i++) {
      int a, b;
      cin >> a >> b;
      a--;b--;
      mp[{min(a, b), max(a, b)}] = i;
      t[a].push_back(b);
      t[b].push_back(a);
      if(t[a].size() > 2 || t[b].size() > 2) {
        f = false;
      }
    }
    if(!f) {
      res[idx] = {-1};
      continue;
    }
    int now = -1;
    for(int i=0;i<n;i++) {
      if(t[i].size() == 1) {
        now = i;
        break;
      }
    }
    vector<int> ans(n-1);
    int par = -1;
    int prev = -1;
    for(int i=0;i<n-1;i++) {
      if(par == -1) {
        int nxt = t[now][0];
        ans[mp[{min(now, nxt), max(now, nxt)}]] = 2;
        prev = 2;
        par = now;
        now = nxt;
      } else {
        int nxt = -1;
        for(auto e : t[now]) {
          if(e != par) nxt = e;
        }
        if(prev == 2) ans[mp[{min(now, nxt), max(now, nxt)}]] = 3;
        else ans[mp[{min(now, nxt), max(now, nxt)}]] = 2;
        prev = ans[mp[{min(now, nxt), max(now, nxt)}]];
        par = now;
        now = nxt;
      }
    }
    res[idx] = ans;
  }
  for(auto e : res) {
    for(int i=0;i<e.size();i++) {
      cout << e[i];
      if(i == e.size()-1) cout << endl;
      else cout << " ";
    }
  }
  return 0;
}
