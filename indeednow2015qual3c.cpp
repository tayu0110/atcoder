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
  Graph t(n+1);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  vector<bool> ck(n+1, false);
  priority_queue<int, vector<int>, greater<int>> nt;
  nt.push(1);
  vector<int> ans;
  while(!nt.empty()) {
    int now = nt.top();
    nt.pop();
    if(ck[now]) continue;
    ck[now] = true;
    ans.push_back(now);
    for(int i=0;i<t[now].size();i++) {
      int j = t[now][i];
      if(ck[j]) continue;
      nt.push(j);
    }
  }
  for(int i=0;i<n;i++) {
    cout << ans[i];
    if(i!=n-1) cout << " ";
  }
  cout << endl;
  return 0;
}
