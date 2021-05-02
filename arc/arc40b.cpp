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
  int n, r;
  string s;
  cin >> n >> r >> s;
  vector<int> ans(n, 0);
  int e = -1;
  for(int i=n-1;i>=0;i--){
    if(s[i] == '.') {
      e = i;
      break;
    }
  }
  if(e == -1) {
    cout << 0 << endl;
    return 0;
  }
  e = max(0, e-r+1);
  int now = 0;
  while(now <= e) {
    if(now == e && s[now] == 'o') {
      ans[now]++;
    }
    if(s[now] == '.') {
      ans[now]++;
      for(int i=0;i<r;i++) {
        // ans[now+i] = k + i + 1;
        s[now+i] = 'o';
      }
      // now = now+r-1;
    }
    // } else {
      ans[now+1] = ans[now] + 1;
    // }
    now++;
  }
  // for(auto k: ans) cout << k << " ";
  // cout << endl;
  cout << ans[e] << endl;
  return 0;
}
