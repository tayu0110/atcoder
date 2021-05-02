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
  int n;
  cin >> n;
  vector<int> p(n);
  for(int i=0;i<n;i++) {
    cin >> p[i];
  }
  vector<pair<int, pii>> d(0), q(0);
  int mx = 0;
  int ud = 0;
  int prev = 0;
  int ans = 1;
  int u = 0;
  int b = 0;
  set<int> ck;
  for(int i=0;i<n;i++) {
    if(i==0) {
      continue;
    } else if(i==n-1) {
      if(mx == ud) ans++;
      else if(mx < ud) {
        mx = ud;
        ans = 1;
      }
    } else {
      if(p[i] > p[i+1] && p[i] > p[i-1]){
        if(mx == u) {
          ck.insert(i);
        } else if(mx < u) {
          mx = u;
          ck.clear();
          ck.insert(i);
        }
        u = 0;
        prev = i;
      } else if(p[i] < p[i+1] && p[i] < p[i-1]) {
        if(mx == b) {
          ck.insert(prev);
        } else if(mx < b) {
          mx = b;
          ck.clear();
          ck.insert(prev);
        }
        b = 0;
        prev = i;
      } else if(p[i] < p[i+1] && p[i] > p[i-1]) {
        u++;
      }else {
        b++;
      }
    }
  }
  if(mx % 2 == 0) cout << 0 << endl;
  else cout << ck.size() << endl;
  return 0;
}
