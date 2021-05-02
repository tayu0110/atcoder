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
  int t;
  cin >> t;
  for(int i=0;i<t;i++) {
    int n, k;
    cin >> n >> k;
    vector<int> ans(0);
    set<int> ck;
    for(int i=n;i>0;i--) {
      if(ck.find(k-i) != ck.end()) continue;
      if(i == k) continue;
      ck.insert(i);
      ans.emplace_back(i);
    }
    cout << ans.size() << endl;
    for(int i=0;i<ans.size();i++) cout << ans[i] << " ";
    cout << endl;
  }
  return 0;
}
