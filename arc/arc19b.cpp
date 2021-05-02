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
  string a;
  cin >> a;
  if(a.size() == 1) {
    cout << 0 << endl;
    return 0;
  }
  int n = a.size();
  vector<bool> ck(n, true);
  int d = 0;
  for(int i=0;i<n-i;i++) {
    if(a[i] != a[n-1-i]) {
      ck[i] = false;
      ck[n-1-i] = false;
      d++;
    }
  }
  if(d == 0) {
    if(n % 2 == 0) cout << 25 * n << endl;
    else cout << 25 * (n-1) << endl;
  } else if(d == 1) {
    cout << 24 * 2 + 25 * (n-2) << endl;
  } else {
    cout << 25 * n << endl;
  }
  return 0;
}
