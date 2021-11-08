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
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  const int m = 1<<9;
  vector<int> mp(m, -inf);
  mp[0] = -1;
  for(int i=0;i<n;i++) {
    int now = a[i];
    for(int j=0;j<m;j++) {
      if(mp[j] == -inf) continue;
      if(mp[j] >= now) continue;
      int nt = j ^ now;
      if(mp[nt] == -inf) mp[nt] = now;
      else mp[nt] = min(mp[nt], now);
    }
  }
  vector<int> res;
  for(int i=0;i<m;i++) if(mp[i] != -inf) res.push_back(i);
  cout << res.size() << endl;
  for(int i=0;i<res.size();i++) {
    cout << res[i];
    if(i == res.size()-1) cout << endl;
    else cout << " ";
  }
  return 0;
}
