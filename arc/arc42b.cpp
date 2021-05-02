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
  Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int x, y;
  cin >> x >> y;
  int n;
  cin >> n;
  vector<pii> xy(n);
  for(int i=0;i<n;i++) {
    int X, Y;
    cin >> X >> Y;
    X -= x;
    Y -= y;
    xy[i].first = X;
    xy[i].second = Y;
  }
  xy.push_back(xy[0]);
  double minv = inf;
  for(int i=0;i<n;i++) {
    double x1 = xy[i].first, y1 = xy[i].second;
    double x2 = xy[i+1].first, y2 = xy[i+1].second;
    double d = abs(x2*y1 - y2*x1)/sqrt((y2-y1)*(y2-y1)+(x2-x1)*(x2-x1));
    minv = min(minv, d);
  }
  cout << minv << endl;
  return 0;
}
