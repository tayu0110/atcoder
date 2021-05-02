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
#define INF (1LL<<60)
#define inf (1<<29)

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<string> s(n);
  vector<int> ck(n);
  vector<vector<ll>> d(2, vector<ll>(n, 0));
  vector<int> f(n);
  for(int i=0;i<n;i++) {
    cin >> s[i];
    int k = 0;
    int l = 1;
    int p = 0;
    for(int j=s[i].size()-1;j>=0;j--) {
      if(s[i][j] == '1') {
        k += l;
        p++;
      }
      l *= 2;
    }
    p %= 2;
    if(i == 0) {
      d[p][i]++;
    } else {
      d[p][i] = d[p][i-1] + 1;
      d[1-p][i] = d[1-p][i-1];
    }
    f[i] = p;
    ck[i] = k;
  }
  ll ans = 0;
  for(int i=0;i<n;i++) {
    int p = 1 - f[i];
    ans += d[p][i];
  }
  // for(int i=0;i<n;i++) {
  //   for(int j=i+1;j<n;j++) {
  //     int k = ck[i] ^ ck[j];
  //     int p = 0;
  //     for(int l=0;l<m;l++) {
  //       if(k & (1<<l)) p++;
  //     }
  //     if(p % 2 == 1) ans++;
  //   }
  // }
  cout << ans << endl;
  return 0;
}
