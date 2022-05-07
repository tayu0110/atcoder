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
  int n, m;
  cin >> n >> m;
  vector<pii> res;
  set<int> ck;
  for(int i=1;i<=n;i++) {
    res.push_back({i, 1});
    ck.insert(3*i+1);
  }
  for(int i=2;i<=m;i++) {
    for(int j=0;j<3;j++) {
      int k = n-j;
      if(ck.find(3*k+i) == ck.end()) {
        ck.insert(3*k+i);
        res.push_back({k, i});
      } else {
        int y = i+1;
        k -= 3;
        while(k > 0 && y <= m) {
          if(ck.find(3*k+y) == ck.end()) {
            res.push_back({k, y});
            ck.insert(3*k+y);
            break;
          }
          k -= 3;
          y++;
        }
      }
    }
  }
  cout << res.size() << endl;
  for(auto [x, y] : res) cout << x << " " << y << endl;
  return 0;
}
