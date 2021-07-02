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

using namespace std;

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

ll solve(vector<ll>& r, vector<ll>& g, vector<ll>& b) {
  ll res = INF;
  for(int i=0;i<g.size();i++) {
    auto it = lower_bound(b.begin(), b.end(), g[i]);
    if(it != b.end()) {
      res = min(res, abs(g[i]-*it));
    }
    if(it == b.begin()) continue;
    it--;
    res = min(res, abs(g[i]-*it));
  }
  for(int i=0;i<b.size();i++) {
    auto it = lower_bound(g.begin(), g.end(), b[i]);
    if(it != g.end()) {
      res = min(res, abs(b[i]-*it));
    }
    if(it == g.begin()) continue;
    it--;
    res = min(res, abs(b[i]-*it));
  }
  vector<pll> gr, br;
  for(int i=0;i<r.size();i++) {
    ll t = INF;
    auto it = lower_bound(g.begin(), g.end(), r[i]);
    if(it != g.end()) t = min(t, abs(r[i]-*it));
    if(it != g.begin()) {
      it--;
      t = min(t, abs(r[i]-*it));
    }
    gr.push_back({t, i});
    t = INF;
    auto it2 = lower_bound(b.begin(), b.end(), r[i]);
    if(it2 != b.end()) t = min(t, abs(r[i]-*it2));
    if(it2 != b.begin()) {
      it2--;
      t = min(t, abs(r[i]-*it2));
    }
    br.push_back({t, i});
  }
  sort(gr.begin(), gr.end());
  sort(br.begin(), br.end());
  ll res2 = INF;
  int i = 0;
  int j = 0;
  while(res2 == INF && i < gr.size() && j < br.size()) {
    if(gr[i].second != br[j].second) {
      res2 = gr[i].first + br[j].first;
      break;
    }
    if(i+1 == gr.size() && j+1 == br.size()) break;
    else if(i+1 == gr.size()) j++;
    else if(j+1 == br.size()) i++;
    else if(gr[i].first+br[j+1].first < gr[i+1].first+br[j].first) j++;
    else i++;
  }
  return min(res, res2);
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  int rn = 0, gn = 0, bn = 0;
  vector<ll> r, g, b;
  for(int i=0;i<2*n;i++) {
    ll a;
    char c;
    cin >> a >> c;
    if(c == 'R') {
      rn++;
      r.push_back(a);
    } else if(c == 'G') {
      gn++;
      g.push_back(a);
    } else {
      bn++;
      b.push_back(a);
    }
  }
  if(rn % 2 == 0 && gn % 2 == 0 && bn % 2 == 0) {
    cout << 0 << endl;
    return 0;
  }
  sort(r.begin(), r.end());
  sort(g.begin(), g.end());
  sort(b.begin(), b.end());
  ll ans = 0;
  if(rn % 2 == 0) {
    ans = solve(r, g, b);
  } else if(gn % 2 == 0) {
    ans = solve(g, r, b);
  } else {
    ans = solve(b, r, g);
  }
  cout << ans << endl;
  return 0;
}
