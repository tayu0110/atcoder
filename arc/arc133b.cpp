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
  cin >> n;
  vector<int> p(n), q(n);
  for(int i=0;i<n;i++) cin >> p[i];
  for(int i=0;i<n;i++) cin >> q[i];
  int qmx = *max_element(q.begin(), q.end());
  vector<vector<int>> v(qmx+1);
  for(int i=0;i<n;i++) {
    int t = q[i];
    for(int j=1;j*j<=t;j++) {
      if(t % j) continue;
      v[j].push_back(i);
      v[t/j].push_back(i);
    }
  }
  vector<int> lis;
  for(int i=0;i<n;i++) {
    int t = p[i];
    if(!v[t].size()) continue;
    if(lis.empty()) {
      lis.push_back(v[t][0]);
      continue;
    }
    for(int j=v[t].size()-1;j>=0;j--) {
      int k = v[t][j];
      auto it = lower_bound(lis.begin(), lis.end(), k);
      if(it == lis.end()) lis.push_back(k);
      else *it = k;
    }
  }
  cout << lis.size() << endl;
  return 0;
}
