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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<queue<int>> q(m);
  vector<pii> p(n);
  for(int i=0;i<m;i++) {
    int k;
    cin >> k;
    set<int> ck;
    for(int j=0;j<k;j++) {
      int a;
      cin >> a;
      a--;
      if(ck.find(a) != ck.end()) {
        cout << "No" << endl;
        return 0;
      }
      ck.insert(a);
      q[i].push(a);
      p[a] = {p[a].second, i};
    }
  }
  queue<int> nt;
  set<int> ck;
  for(int i=0;i<m;i++) {
    int t = q[i].front();
    if(ck.find(t) != ck.end()) {
      nt.push(t);
      ck.erase(t);
    } else ck.insert(t);
  }
  while(!nt.empty()) {
    int t = nt.front();
    nt.pop();
    int f = p[t].first, s = p[t].second;
    q[f].pop(); q[s].pop();
    if(!q[f].empty()) {
      int l = q[f].front();
      if(ck.find(l) != ck.end()) {
        ck.erase(l);
        nt.push(l);
      } else ck.insert(l);
    }
    if(!q[s].empty()) {
      int l = q[s].front();
      if(ck.find(l) != ck.end()) {
        ck.erase(l);
        nt.push(l);
      } else ck.insert(l);
    }
  }
  for(int i=0;i<m;i++) {
    if(!q[i].empty()) {
      cout << "No" << endl;
      return 0;
    }
  }
  cout << "Yes" << endl;
  return 0;
}
