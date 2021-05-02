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
  vector<int> a(n);
  map<int, int> mp;
  set<int> ck;
  int mx = 0;
  for(int i=0;i<n;i++) {
    cin >> a[i];
    mx = max(mx, a[i]);
  }
  for(int i=0;i<mx+2;i++) ck.insert(i);
  int t = 0;
  int ans = inf;
  for(int i=0;i<n;i++) {
    if(t < m) {
      mp[a[i]]++;
      if(ck.find(a[i]) !=ck.end())ck.erase(a[i]);
      t++;
      continue;
    }
    auto it = ck.begin();
    ans = min(ans, *it);
    int b= a[i-m];
    if(mp[b] > 1) {
      mp[b]--;
    } else {
      mp[b] = 0;
      ck.insert(b);
    }
    mp[a[i]]++;
    if(ck.find(a[i]) != ck.end())ck.erase(a[i]);
  }
  auto it = ck.begin();
  ans = min(ans, *it);
  cout << ans << endl;
  return 0;
}
