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
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, x;
  cin >> n >> x;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    int a, b;
    cin >> a >> b;
    p[i] = {a, b};
  }
  set<int> ck;
  ck.insert(0);
  for(int i=0;i<n;i++) {
    set<int> tmp;
    auto [a, b] = p[i];
    for(auto e : ck) {
      tmp.insert(e+a);
      tmp.insert(e+b);
    }
    tmp.swap(ck);
  }
  if(ck.find(x) != ck.end()) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
