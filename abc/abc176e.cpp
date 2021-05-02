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

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w, m;
  cin >> h >> w >> m;
  vector<int> x(w), y(h);
  set<pii> ck;
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    x[b]++;
    y[a]++;
    ck.insert(make_pair(a, b));
  }
  int hmx = 0, wmx = 0;
  vector<int> hp, wp;
  for(int i=0;i<h;i++) hmx = max(hmx, y[i]);
  for(int i=0;i<h;i++) if(hmx == y[i]) hp.push_back(i);
  for(int i=0;i<w;i++) wmx = max(wmx, x[i]);
  for(int i=0;i<w;i++) if(wmx == x[i]) wp.push_back(i);
  for(int i=0;i<hp.size();i++) {
    for(int j=0;j<wp.size();j++) {
      if(ck.find(make_pair(hp[i], wp[j])) == ck.end()) {
        cout << hmx + wmx << endl;
        return 0;
      }
    }
  }
  cout << hmx + wmx - 1 << endl;
  return 0;
}
