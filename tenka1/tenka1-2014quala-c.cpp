#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<string> p(n);
  for(int i=0;i<n;i++) cin >> p[i];
  set<pii> ck;
  for(int i=0;i<n;i++) for(int j=i+1;j<n;j++) {
    string &a = p[i];
    string &b = p[j];
    bool f = true;
    for(int k=0;k<m;k++) {
      if(a[k] != '*' && b[k] != '*' && a[k] != b[k]) f = false;
    }
    if(f) ck.insert({i, j});
  }
  vector<int> dp(1 << n, inf);
  dp[0] = 0;
  for(int i=0;i<(1<<n);i++) {
    bool f = true;
    for(int j=0;j<n;j++) {
      if(!(i & (1 << j))) continue;
      for(int k=j+1;k<n;k++) {
        if(!(i & (1 << k))) continue;
        if(ck.find({j, k}) == ck.end()) f = false;
      }
    }
    if(f) {
      for(int j=0;j<(1<<n);j++) {
        if((i & j) == 0) dp[i|j] = min(dp[i|j], dp[j]+1);
      }
    }
  }
  cout << dp[(1<<n)-1] << endl;
  return 0;
}
