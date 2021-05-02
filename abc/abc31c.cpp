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
  int n;
  cin >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++){
    cin >> a[i];
  }
  int ans = -inf;
  for(int i=0;i<n;i++){
    int amax = -inf;
    int t = -inf;
    for(int j=0;j<n;j++){
      if(i==j) continue;
      int tp = 0;
      int ap = 0;
      int l = min(i, j);
      int r = max(i, j);
      for(int k=l;k<=r;k++){
        if((k-l)%2 == 0) tp += a[k];
        else ap += a[k];
      }
      if(ap > amax) {
        amax = ap;
        t = tp;
      }
    }
    ans = max(ans, t);
  }
  cout << ans << endl;
  return 0;
}
