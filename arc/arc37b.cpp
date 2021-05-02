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
  int n, m;
  cin >> n >> m;
  Graph t(n+1);
  set<pii> ck;
  for(int i=0;i<m;i++){
    int u, v;
    cin >> u >> v;
    t[u].push_back(v);
    t[v].push_back(u);
    ck.insert(make_pair(u, v));
    ck.insert(make_pair(v, u));
  }
  vector<bool> r(n+1, false);
  int ans = 0;
  for(int i=1;i<n+1;i++){
    if(r[i]) continue;
    queue<int> nt;
    nt.push(i);
    bool flag = true;
    while(!nt.empty()) {
      int now = nt.front();
      nt.pop();
      if(r[now]) flag = false;
      r[now] = true;
      for(int j=0;j<t[now].size();j++){
        int k = t[now][j];
        if(ck.find(make_pair(now, k)) == ck.end()) continue;
        nt.push(k);
        ck.erase(make_pair(now, k));
        ck.erase(make_pair(k, now));
      }
    }
    if(flag) {
      ans++;
      // cout << "i: " << i << endl;
    }
  }
  cout << ans << endl;
  return 0;
}
