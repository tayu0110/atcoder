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
  ll k;
  cin >> n >> k;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<ll> b(n+1);
  for(int i=0;i<n;i++) {
    b[i+1] = a[i] + b[i];
    b[i+1] %= k;
  }
  map<ll, ll> mp;
  for(int i=0;i<n+1;i++) {
    b[i] -= i;
    if(b[i] < 0) b[i] += (abs(b[i]) / k + 1) * k;
    b[i] %= k;
    if(i < k) mp[b[i]]++;
  }
  print_with_space(b);
  ll ans = 0;
  for(int i=0;i<n+1;i++) {
    ll t = b[i];
    ans += mp[t] - 1;
    mp[t]--;
    if(i+k < n+1) mp[b[i+k]]++;
  }
  cout << ans << endl;
  return 0;
}
