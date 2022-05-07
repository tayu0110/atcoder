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
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> v(n*2+1), id(n*2+1);
  set<int> ck;
  for(int i=0;i<n;i++) {
    int a, b;
    cin >> a >> b;
    if(a >= 0 && b >= 0) {
      if(a >= b || ck.find(a) != ck.end() || ck.find(b) != ck.end()) {
        cout << "No" << endl;
        return 0;
      }
      ck.insert(a);
      ck.insert(b);
      v[a] = b;
      v[b] = a;
      id[a] = i+1;
      id[b] = -(i+1);
    } else if(a >= 0) {
      if(ck.find(a) != ck.end()) {
        cout << "No" << endl;
        return 0;
      }
      ck.insert(a);
      v[a] = inf;
      id[a] = i+1;
    } else if(b >= 0) {
      if(ck.find(b) != ck.end()) {
        cout << "No" << endl;
        return 0;
      }
      ck.insert(b);
      v[b] = -inf;
      id[b] = -(i+1);
    }
  }
  vector<bool> dp(n*2+1, false);
  dp[0] = true;
  for(int i=1;i<n*2+1;i++) {
    if(!dp[i-1]) continue;
    for(int len=2;len<=n*2;len+=2) {
      if(i+len-1 > n*2) break;
      int r = i+len/2;
      bool f = true;
      for(int from=i;from<r;from++) {
        int to = from+len/2;
        if(v[from] == -inf || v[to] == inf) { f = false; break; }
        if(v[from] && v[from] != inf && v[from] != to) { f = false; break; }
        if(v[to] && v[to] != -inf && v[to] != from) { f = false; break; }
        if(id[from] && id[to] && id[from] + id[to] != 0) { f = false; break; }
      }
      dp[i+len-1] = dp[i+len-1] | f;
    }
  }
  if(dp[n*2]) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
