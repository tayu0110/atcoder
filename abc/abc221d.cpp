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
  vector<ll> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i] >> b[i];
  map<ll, int> mp;
  for(int i=0;i<n;i++) {
    mp[a[i]]++;
    mp[a[i]+b[i]]--;
  }
  for(auto it=mp.begin();it!=mp.end();it++) {
    auto nt = it;
    nt++;
    if(nt == mp.end()) break;
    nt->second += it->second;
  }
  vector<int> ans(n+1);
  for(auto it=mp.begin();it!=mp.end();it++) {
    auto nt = it;
    nt++;
    int l = it->first;
    int r = -1;
    if(nt == mp.end()) r = it->first+it->second-1;
    else r = nt->first;
    ans[it->second] += r-l;
  }
  for(int i=1;i<n+1;i++) {
    cout << ans[i];
    if(i != n) cout << " ";
  }
  cout << endl;
  return 0;
}
