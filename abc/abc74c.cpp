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
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int a, b, c, d, e, f;
  cin >> a >> b >> c >> d >> e >> f;
  vector<int> x(0);
  for(int i=0;i<f+1;i++) {
    if(100*a*i > f) continue;
    int k = f - 100*a*i;
    for(int j=0;j<f+1;j++) {
      if(100*b*j > k) continue;
      x.push_back(100*a*i + 100*b*j);
    }
  }
  vector<int> y(0);
  for(int i=0;i<f+1;i++) {
    if(c*i > f) continue;
    int k = f - c*i;
    for(int j=0;j<f+1;j++) {
      if(d*j > k) continue;
      y.push_back(c*i + d*j);
    }
  }
  int w = 100*a;
  int s = 0;
  for(auto xi : x) {
    for(auto yi : y) {
      if(xi == 0 && yi == 0) continue;
      if(xi + yi > f) continue;
      int ne = xi / 100 * e;
      if(yi > ne) continue;
      if(s*(xi+yi) < yi*(w+s)) {
        w = xi;
        s = yi;
      }
    }
  }
  cout << w+s << " " << s << endl;
  return 0;
}
