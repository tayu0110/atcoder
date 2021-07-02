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
  int h, w, k;
  cin >> h >> w >> k;
  vector<vector<int>> s(h, vector<int>(w));
  for(int i=0;i<h;i++) {
    string t;
    cin >> t;
    for(int j=0;j<w;j++) {
      if(t[j] == '1') s[i][j]++;
    }
  }
  vector<vector<int>> ts(h+1, vector<int>(w+1));
  for(int i=1;i<h+1;i++) for(int j=0;j<w;j++) ts[i][j+1] = ts[i][j] + s[i-1][j];
  for(int i=0;i<h;i++) for(int j=1;j<w+1;j++) ts[i+1][j] += ts[i][j];
  int ans = h+w-2;
  for(int i=0;i<(1<<(h-1));i++) {
    vector<int> q;
    for(int j=0;j<h-1;j++) if(i & (1<<j)) q.push_back(j+1);
    int pw = 0;
    int cnt = q.size();
    q.push_back(h);
    for(int j=1;j<w+1;j++) {
      int ph = 0;
      for(auto l : q) {
        int c = ts[l][j] - ts[l][pw] - ts[ph][j] + ts[ph][pw];
        if(c > k) {
          if(j-1 == pw) cnt = inf;
          pw = j-1;
          j--;
          cnt++;
        }
        ph = l;
      }
      if(cnt >= inf) break;
    }
    ans = min(ans, cnt);
  }
  cout << ans << endl;
  return 0;
}
