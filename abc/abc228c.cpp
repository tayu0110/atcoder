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
  int n, k;
  cin >> n >> k;
  vector<vector<int>> p(n, vector<int>(3, 0));
  for(int i=0;i<n;i++) {
    for(int j=0;j<3;j++) cin >> p[i][j];
  }
  vector<int> t(n);
  for(int i=0;i<n;i++) for(int j=0;j<3;j++) t[i] += p[i][j];
  vector<int> q = t;
  sort(q.begin(), q.end(), greater<int>());
  int b = q[k-1];
  for(int i=0;i<n;i++) {
    if(t[i] + 300 >= b) cout << "Yes" << endl;
    else cout << "No" << endl;
  }
  return 0;
}