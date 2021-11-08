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
#include<cassert>

using namespace std;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
vector<int> d;
void dfs(int now, int par, int nd, Graph &t) {
  d[now] = nd;
  for(auto e : t[now]) {
    if(e == par) continue;
    if(d[e] >= 0) continue;
    dfs(e, now, nd+1, t);
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  d.assign(n, -1);
  dfs(0, -1, 0, t);
  int mx = *max_element(d.begin(), d.end());
  int idx = find(d.begin(), d.end(), mx) - d.begin();
  d.assign(n, -1);
  dfs(idx, -1, 0, t);
  mx = *max_element(d.begin(), d.end());
  if(mx % 3 == 1) {
    cout << "Second" << endl;
  } else {
    cout << "First" << endl;
  }
  return 0;
}
