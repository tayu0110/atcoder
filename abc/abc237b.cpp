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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

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
const ld PI = acos(-1);

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  string s;
  cin >> n >> s;
  vector<int> prev(n+1, -1), nxt(n+1, -1);
  for(int i=1;i<n+1;i++) {
    if(s[i-1] == 'L') {
      int p = prev[i-1];
      if(p < 0) {
        prev[i-1] = i;
        nxt[i] = i-1;
        continue;
      }
      nxt[i] = nxt[p];
      nxt[p] = i;
      prev[i-1] = i;
      prev[i] = p;
    } else {
      int nt = nxt[i-1];
      if(nt < 0) {
        nxt[i-1] = i;
        prev[i] = i-1;
        continue;
      }
      prev[i] = prev[nt];
      prev[nt] = i;
      nxt[i-1] = i;
      nxt[i] = nt;
    }
  }
  int now = -1;
  for(int i=0;i<n+1;i++) if(prev[i] < 0) now = i;
  for(int i=0;i<n+1;i++) {
    cout << now;
    if(i == n) cout << endl;
    else {
      cout << " ";
      now = nxt[now];
    }
  }
  return 0;
}
