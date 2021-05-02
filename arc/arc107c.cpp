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

ll perm(ll n) {
  ll res = 1;
  while(n > 1) {
    res *= n;
    res %= 998244353;
    n--;
  }
  return res;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, K;
  cin >> n >> K;
  vector<vector<int>> a(n, vector<int>(n));
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      cin >> a[i][j];
    }
  }
  vector<vector<bool>> h(n, vector<bool>(n, true)), w(n, vector<bool>(n, true));
  UnionFind ufh(n), ufw(n);
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      for(int k=0;k<n;k++) {
        for(int l=0;l<n;l++) {
          if(i==k && j==l) continue;
          if(i==k && a[i][j] + a[k][l] > K) {
            w[j][l] = false;
          }
          if(j==l && a[i][j] + a[k][l] > K) {
            h[i][k] = false;
          }
        }
      }
    }
  }
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      if(i==j) continue;
      if(h[i][j]) ufh.merge(i, j);
      if(w[i][j]) ufw.merge(i, j);
    }
  }
  ll hans = 1, wans = 1;
  for(int i=0;i<n;i++) {
    if(ufh.root(i) == i) hans *= perm(ufh.size(i));
    if(ufw.root(i) == i) wans *= perm(ufw.size(i));
    hans %= 998244353;
    wans %= 998244353;
  }
  cout << hans * wans % 998244353 << endl;
  return 0;
}
