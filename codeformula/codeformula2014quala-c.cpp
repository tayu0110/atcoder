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
  int n, k;
  cin >> n >> k;
  vector<vector<int>> a(n, vector<int>(k));
  for(int i=0;i<n;i++) {
    for(int j=0;j<k;j++) cin >> a[i][j];
  }
  vector<int> sp(k, 0);
  vector<vector<int>> ans(n);
  set<int> ck;
  int rem = k;
  for(int i=0;i<n;i++) {
    int sum = 0;
    for(int j=0;j<k;j++) {
      while(sp[j] <= i) {
        if(ck.find(a[sp[j]][j]) != ck.end()) {
          sp[j]++;
          continue;
        }
        if(sum < rem) {
          ck.insert(a[sp[j]][j]);
          ans[i].push_back(a[sp[j]][j]); 
          rem--;
          sp[j]++;
        } else break;
      }
      sum += n-sp[j];
    }
  }
  for(int i=0;i<n;i++) {
    sort(ans[i].begin(), ans[i].end());
    int sz = ans[i].size();
    for(int j=0;j<sz;j++) {
      cout << ans[i][j];
      if(j != sz-1) cout << " ";
    }
    cout << endl;
  }
  return 0;
}
