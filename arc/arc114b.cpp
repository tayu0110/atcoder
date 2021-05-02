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

struct UnionFind{
  vector<int> par;

  UnionFind(int n){
    par = vector<int>(n, -1);
  }
  int root(int x){
    if(par[x] < 0) return x;
    return par[x] = root(par[x]);
  }
  bool merge(int x, int y){
    int rx = root(x);
    int ry = root(y);
    if(rx == ry) return false;
    if(par[rx] > par[ry]) swap(x, y);
    par[rx] += par[ry];
    par[ry] = rx;
    return true;
  }
  bool isSame(int x, int y){
    int rx = root(x);
    int ry = root(y);
    return rx == ry;
  }
  int size(int x){
    return -par[root(x)];
  }
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> f(n);
  UnionFind uf(n);
  map<int, int> mp;
  for(int i=0;i<n;i++) {
    cin >> f[i];
    f[i]--;
    if(mp.find(f[i]) != mp.end()) uf.merge(i, mp[f[i]]);
    else mp[f[i]] = i;
  }
  for(int i=0;i<n;i++) {
    int a = i;
    int b = f[i];
    uf.merge(a, b);
  }
  ll sz = 0;
  for(int i=0;i<n;i++) {
    if(uf.root(i) == i) sz++;
  }
  ll ans = 1;
  while(sz--) {
    ans *= 2;
    ans %= 998244353;
  }
  if(ans == 0) ans += 998244353;
  cout << ans -1 << endl;
  return 0;
}
