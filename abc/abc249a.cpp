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
  int a, b, c, d, e, f, x;
  cin >> a >> b >> c >> d >> e >>f >> x;
  int s = x;
  int resa = 0, resb = 0;
  while(x) {
    if(x >= a+c) {
      resa += a * b;
      x -= a + c;
    } else if(x >= a) {
      resa += a * b;
      x = 0;
    } else {
      resa += x * b;
      x = 0;
    }
  }
  while(s) {
    if(s >= d+f) {
      resb += d * e;
      s -= d + f;
    } else if(s >= d) {
      resb += d * e;
      s = 0;
    } else {
      resb += s * e;
      s = 0;
    }
  }
  if(resa > resb) {
    cout << "Takahashi" << endl;
  } else if(resa < resb) {
    cout << "Aoki" << endl;
  } else {
    cout << "Draw" << endl;;
  }
  return 0;
}
