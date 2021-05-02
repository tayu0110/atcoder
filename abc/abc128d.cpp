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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<ll> v(n);
  for(int i=0;i<n;i++) {
    cin >> v[i];
  }
  ll mx = 0;
  for(int a=0;a<=min(n, k);a++) {
    for(int b=0;a+b<=min(n, k);b++) {
      ll tot = 0;
      vector<ll> neg;
      for(int i=0;i<a;i++) {
        tot += v[i];
        if(v[i] < 0) neg.push_back(v[i]);
      }
      for(int i=n-1;i>n-b-1;i--) {
        tot += v[i];
        if(v[i] < 0) neg.push_back(v[i]);
      }
      sort(neg.begin(), neg.end());
      for(int t=0;t<min(k-(a+b), (int)neg.size());t++) {
        tot -= neg[t];
      }
      mx = max(mx, tot);
    }
  }
  cout << mx << endl;
  return 0;
}
