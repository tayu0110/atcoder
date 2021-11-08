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

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string s;
  int x, y;
  cin >> s >> x >> y;
  int n = s.length();
  vector<int> h, v;
  bool hor = true;
  bool first_of = true;
  int ff = 0;
  int f = 0, t = 0;
  for(int i=0;i<n;i++) {
    if(s[i] == 'T') {
      if(f > 0) {
        if(hor) {
          if(!first_of) h.push_back(f);
          else ff = f;
        } else v.push_back(f);
        f = 0;
      }
      first_of = false;
      t++;
    } else {
      if(t > 0) {
        if(t % 2 == 1) hor = !hor;
        t = 0;
      }
      f++;
    }
  }
  if(f > 0) {
    if(hor) {
      if(!first_of) h.push_back(f);
      else ff = f;
    } else v.push_back(f);
  }
  vector<bool> hdp(2*n+1, false), vdp(2*n+1, false);
  hdp[n+ff] = true;
  vdp[n] = true;
  for(int i=0;i<h.size();i++) {
    vector<int> minp;
    for(int j=2*n;j>=0;j--) {
      if(hdp[j]) {
        if(j+h[i] < hdp.size()) hdp[j+h[i]] = true;
        if(j-h[i] >= 0) minp.push_back(j-h[i]);
        hdp[j] = false;
      }
    }
    for(auto e : minp) hdp[e] = true;
  }
  for(int i=0;i<v.size();i++) {
    vector<int> minp;
    for(int j=2*n;j>=0;j--) {
      if(vdp[j]) {
        if(j+v[i] < vdp.size()) vdp[j+v[i]] = true;
        if(j-v[i] >= 0) minp.push_back(j-v[i]);
        vdp[j] = false;
      }
    }
    for(auto e : minp) vdp[e] = true;
  }
  if(hdp[n+x] && vdp[n+y]) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
