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
  int n;
  cin >> n;
  vector<int> p(n+1);
  for(int i=1;i<n+1;i++) cin >> p[i];
  set<pii> ck;
  for(int i=2;i<n+1;i++) {
    int t = p[i];
    vector<int> q;
    for(int j=1;j*j<=i;j++) {
      if(i % j) continue;
      if(j != 1) q.push_back(j);
      if(j * j != i) q.push_back(i / j);
    }
    for(auto e : q) {
      for(int j=i/e;e*j<n+1;j++) {
        if(gcd(t, p[e*j]) != 1) {
          ck.insert({i, e*j});
        }
      }
    }
  }
  cout << ck.size() << endl;
  return 0;
}
