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
bool flag;
int sl(vector<int> &a) {
  int g = -1;
  int n = a.size();
  for(int i=0;i<n;i++) for(int j=i+1;j<n;j++) {
    if(i == j) continue;
    if(a[i] == a[j]) continue;
    if(g < 0) g = abs(a[i] - a[j]);
    else g = gcd(g, abs(a[i]-a[j]));
  }
  return g;
}
void half(vector<int> &a, vector<pll> &t) {
  int n = a.size();
  for(int i=0;i<(1<<n);i++) {
    vector<int> b;
    for(int j=0;j<n;j++) if(i & (1<<j)) b.push_back(a[j]);
    int g = sl(a);
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<int> res;
  while(t--) {
    flag = false;
    int n;
    cin >> n;
    vector<int> a(n/2), b(n/2);
    for(int i=0;i<n/2;i++) cin >> a[i];
    for(int i=0;i<n/2;i++) cin >> b[i];
    vector<pll> c, d;

  }
  return 0;
}
