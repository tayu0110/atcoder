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

vector<bool> ck;
vector<ll> memo;
ll ans = -INF;

ll dfs(int now, Graph &t, vector<ll> &a) {
  if(t[now].size() == 0) {
    ck[now] = true;
    return memo[now] = a[now];
  }
  if(memo[now] != -INF) return memo[now];
  ck[now] = true;
  ll amx = -INF;
  for(int i=0;i<t[now].size();i++) {
    int j=t[now][i];
    amx = max(amx, dfs(j, t, a));
  }
  ans = max(ans, amx - a[now]);
  return memo[now] = max(amx, a[now]);
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<ll> a(n+1);
  for(int i=1;i<n+1;i++) {
    cin >> a[i];
  }
  Graph t(n+1);
  for(int i=0;i<m;i++) {
    int x, y;
    cin >> x >> y;
    t[x].push_back(y);
  }
  ck.assign(n+1, false);
  memo.assign(n+1, -INF);
  for(int i=1;i<n+1;i++) {
    if(ck[i]) continue;
    dfs(i, t, a);
  }
  cout << ans << endl;
  return 0;
}
