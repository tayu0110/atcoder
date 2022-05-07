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
  int t;
  cin >> t;
  for(int i=1;i<=t;i++) {
    int n;
    cin >> n;
    deque<int> nt;
    for(int j=0;j<n;j++) {
      int d;
      cin >> d;
      nt.push_back(d);
    }
    int ans = 0;
    int prev = -1;
    while(!nt.empty()) {
      if(nt.size() == 1) {
        ans += nt.front() >= prev;
        nt.pop_back();
        continue;
      }
      if(nt.front() < prev) {
        nt.pop_front(); continue;
      }
      if(nt.back() < prev) {
        nt.pop_back(); continue;
      }
      ans++;
      if(nt.front() < nt.back()) {
        prev = nt.front();
        nt.pop_front();
      } else {
        prev = nt.back();
        nt.pop_back();
      }
    }
    printf("Case #%d: %d\n", i, ans);
  }
  return 0;
}
