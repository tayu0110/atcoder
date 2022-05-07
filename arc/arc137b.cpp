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
  int n;
  cin >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<int> b(n+1);
  for(int i=0;i<n;i++) {
    if(a[i] == 1) b[i+1]--;
    else b[i+1]++;
    b[i+1] += b[i];
  }
  multiset<int> mn, mx;
  for(int i=0;i<n+1;i++) {
    mn.insert(b[i]);
    mx.insert(b[i]);
  }
  int ans_min = 0, ans_mx = 0;
  for(int i=0;i<n;i++) {
    int t = b[i];
    mn.erase(mn.find(t));
    mx.erase(mx.find(t));
    {
      auto it = mx.end();
      it--;
      ans_mx = max(ans_mx, *it - t);
    }
    {
      auto it = mn.begin();
      ans_min = min(ans_min, *it - t);
    }
  }
  cout << abs(ans_min) + ans_mx + 1 << endl;
  return 0;
}
