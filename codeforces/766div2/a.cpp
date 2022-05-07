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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<int> res;
  while(t--) {
    int n, m, r, c;
    cin >> n >> m >> r >> c;
    r--;c--;
    vector<string> g(n);
    for(int i=0;i<n;i++) cin >> g[i];
    bool rcf = false;
    bool rf = false, cf = false;
    bool f = false;
    for(int i=0;i<n;i++) for(int j=0;j<m;j++) {
      if(g[i][j] == 'B') {
        f = true;
        if(i == r && j == c) rcf = true;
        if(i == r) rf = true;
        if(j == c) cf = true;
      }
    }
    if(!f) res.push_back(-1);
    else if(rcf) res.push_back(0);
    else if(rf || cf) res.push_back(1);
    else res.push_back(2);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
