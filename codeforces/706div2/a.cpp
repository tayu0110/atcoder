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
  int t;
  cin >> t;
  for(int i=0;i<t;i++) {
    int n, k;
    string s;
    cin >> n >> k >> s;
    if(k == 0) {
      cout << "YES" << endl;
      continue;
    }
    if(n == 2*k) {
      cout << "NO" << endl;
      continue;
    }
    bool ans = true;
    for(int j=0;j<k;j++) {
      int l = n-1-j;
      if(l < j) break;
      if(s[j] != s[l]) {
        ans = false;
        break;
      }
    }
    if(ans) cout << "YES" << endl;
    else cout << "NO" << endl;
  }
  return 0;
}
