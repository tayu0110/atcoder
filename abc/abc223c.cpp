#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

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
  int n;
  cin >> n;
  vector<pair<double, double>> p(n);
  double sum = 0;
  for(int i=0;i<n;i++) {
    double a, b;
    cin >> a >> b;
    p[i] = {a, b};
    sum += a;
  }
  double l = 0, r = 100000010;
  while(r - l > 0.00000001) {
    double m = (r+l) / 2;
    double nl = 0, nr = sum;
    double lm = m, rm = m;
    for(int i=0;i<n;i++) {
      double a = p[i].first, b = p[i].second;
      if(lm * b > a) {
        lm -= a / b;
        nl += a;
      } else {
        nl += lm * b;
        break;
      }
    }
    for(int i=n-1;i>=0;i--) {
      double a = p[i].first, b = p[i].second;
      if(rm * b > a) {
        rm -= a / b;
        nr -= a;
      } else {
        nr -= rm * b;
        break;
      }
    }
    if(nl < nr) l = m;
    else r = m;
  }
  double now = 0;
  for(int i=0;i<n;i++) {
    double a = p[i].first, b = p[i].second;
    if(l * b > a) {
      l -= a / b;
      now += a;
    } else {
      now += l * b;
      break;
    }
  }
  cout << now << endl;
  return 0;
}
