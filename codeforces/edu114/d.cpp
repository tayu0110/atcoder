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
  int n;
  cin >> n;
  vector<vector<ll>> a(n);
  for(int i=0;i<n;i++) {
    int c;
    cin >> c;
    for(int j=0;j<c;j++) {
      ll k;
      cin >> k;
      a[i].push_back(k);
    }
  }
  int m;
  cin >> m;
  set<ll> ck;
  for(int i=0;i<m;i++) {
    ll s = 0;
    for(int j=0;j<n;j++) {
      ll t;
      cin >> t;
      t--;
      s = s * 10 + t;
    }
    ck.insert(s);
  }
  vector<int> pos(n);
  for(int i=0;i<n;i++) {
    pos[i] = a[i].size()-1;
  }
  for(int i=0;i<m+1;i++) {
    ll s = 0;
    for(auto e : pos) {
      s = s * 10 + e;
    }
    if(ck.find(s) == ck.end()) {
      for(int j=0;j<n;j++) {
        cout << pos[j]+1;
        if(j != n-1) cout << " ";
      }
      cout << endl;
      return 0;
    }
    int idx = -1;
    ll mn = INF;
    for(int j=0;j<n;j++) {
      if(pos[j] == 0) continue;
      int p = pos[j];
      if(a[j][p] - a[j][p-1] < mn) {
        mn = a[j][p] - a[j][p-1];
        idx = j;
      }
    }
    pos[idx]--;
  }
  return 0;
}
