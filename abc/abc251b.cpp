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
  int n, w;
  cin >> n >> w;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  set<int> ck;
  for(int i=0;i<n;i++) {
    if(a[i] <= w) ck.insert(a[i]);
    for(int j=i+1;j<n;j++) {
      if(a[i] + a[j] <= w) ck.insert(a[i] + a[j]);
      for(int k=j+1;k<n;k++) {
        if(a[i] + a[j] + a[k] <= w) ck.insert(a[i] + a[j] + a[k]);
      }
    }
  }
  cout << ck.size() << endl;
  return 0;
}
