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
#include <random>
#include <bitset>
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
  random_device rd;
  vector<default_random_engine> eng(9, default_random_engine(rd()));
  vector<vector<int>> v(9);
  for(int i=1;i<(1<<8);i++) v[__builtin_popcount(i)].push_back(i);
  vector<uniform_int_distribution<int>> distr(9);
  for(int i=1;i<9;i++) distr[i] = uniform_int_distribution<int>(0, v[i].size()-1);
  while(t--) {
    while(true) {
      int n;
      cin >> n;
      if(!n) break;
      if(n == 8) cout << "11111111" << endl;
      else cout << "00000001" << endl;
    }
  }
  return 0;
}
