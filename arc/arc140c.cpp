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

int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int n, x;
  cin >> n >> x;
  set<int> ck;
  vector<int> s = {x};
  ck.insert(x);
  if(x != (n+1)/2) {
    s.push_back((n+1)/2);
    ck.insert((n+1)/2);
  }
  int prev = (n+1)/2;
  int l = 1;
  for(int i=1;i<=n-1;i++) {
    int k = i%2 ? prev + i*l : prev - i*l;
    if(ck.find(k) == ck.end()) {
      s.push_back(k);
      l = -1;
    }
    prev = k;
    ck.insert(k);
  }
  for(int i=0;i<n;i++) {
    if(i) cout << " "; cout << s[i];
  }
  cout << endl;
  return 0;
}
