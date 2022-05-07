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
  int l;
  cin >> l;
  int cnt = 1;
  int now = 1;
  while(now <= l) now <<= 1, cnt++;
  cnt--;
  vector<tuple<int, int, int>> p;
  for(int i=0;i<cnt-1;i++) {
    p.push_back({cnt-i-1, cnt-i, 1<<i});
    p.push_back({cnt-i-1, cnt-i, 0});
  }
  for(int i=0;i<cnt-1;i++) if(l & (1<<i)) {
    int t = l - (l % (1<<i)) - (1<<i);
    p.push_back({1, cnt-i, t});
  }
  cout << cnt << " " << p.size() << endl;
  for(auto [from, to, w] : p) {
    cout << from << " " << to << " " << w << endl;
  }
  return 0;
}
