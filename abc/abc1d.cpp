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
int myfloor(int t) {
  int d = t % 10;
  if(d > 5) t -= d - 5;
  if(d < 5) t -= d;
  return t;
}
int myceil(int t) {
  int d = t % 10;
  if(d > 5) t += 10 - d;
  if(0 < d && d < 5) t += 5 - d;
  return t;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    string str;
    cin >> str;
    string sstr, estr;
    bool f = false;
    for(int j=0;j<str.length();j++) {
      if(str[j] == '-') {
        f = true;
        continue;
      }
      if(f) estr += str[j];
      else sstr += str[j];
    }
    int s = stoi(sstr), e = stoi(estr);
    s = myfloor(s);
    if(e % 100 > 55) e += 100 - e % 100;
    else e = myceil(e);
    p[i] = {s, e};
  }
  sort(p.begin(), p.end());
  deque<pii> nt;
  for(int i=0;i<n;i++) {
    pii now = p[i];
    while(!nt.empty() && nt.back().second >= now.first) {
      pii prev = nt.back();
      nt.pop_back();
      now = {prev.first, max(now.second, prev.second)};
    }
    nt.push_back(now);
  }
  while(!nt.empty()) {
    string s = to_string(nt.front().first), e = to_string(nt.front().second);
    while(s.length() < 4) s = "0" + s;
    while(e.length() < 4) e = "0" + e;
    cout << s << "-" << e << endl;
    nt.pop_front();
  }
  return 0;
}
