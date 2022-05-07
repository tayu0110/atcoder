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
const ld PI = 3.141592653589793238462643383;

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  Graph a(n);
  for(int i=0;i<m;i++) {
    int c, d;
    cin >> c >> d;
    c--;d--;
    a[c].push_back(d);
    a[d].push_back(c);
  }
  vector<int> p(n);
  for(int i=0;i<n;i++) p[i] = i;
  int cnt = 0;
  do {
    bool f = true;
    for(int i=0;i<n;i++) for(int j=-0;j<n;j++) {
      int pi = p[i];
      int pj = p[j];
      bool tf = false;
      for(auto e : t[i]) if(e == j) tf = true;
      bool af = false;
      for(auto e : a[pi]) if(e == pj) af = true;
      if(tf != af) {
        f = false;
        break;
      }
    }
    if(f) {
      cout << "Yes" << endl;
      return 0;
    }
  } while(next_permutation(p.begin(), p.end()));
  cout << "No" << endl;
  return 0;
}
