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
  int n, q;
  string s;
  cin >> n >> q >> s;
  // vector<pii> v(n);
  // RMinRSumQuery st(n);
  // for(int i=0;i<n;i++) {
  //   v[i] = s[i] == '(' ? make_pair(0, 1) : make_pair(-1, -1);
  //   st.update(i, v[i]);
  // }
  // while(q--) {
  //   int t, l, r;
  //   cin >> t >> l >> r;
  //   l--; r--;
  //   if(t == 1) {
  //     if(v[l] == v[r]) continue;
  //     swap(v[l], v[r]);
  //     st.update(l, v[l]);
  //     st.update(r, v[r]);
  //     // DEBUG(v[l].first);DEBUG(v[l].second);DEBUG(v[r].first);DEBUG_EN(v[r].second);
  //   } else {
  //     // DEBUG(st.get(l, r+1).first);DEBUG_EN(st.get(l, r+1).second);
  //     if(st.get(l, r+1) == make_pair(0LL, 0LL)) cout << "Yes" << endl;
  //     else cout << "No" << endl;
  //   }
  // }
  return 0;
}
