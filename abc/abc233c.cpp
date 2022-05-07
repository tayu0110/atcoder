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
  ll n, x;
  cin >> n >> x;
  vector<vector<ll>> a(n);
  for(int i=0;i<n;i++) {
    int l;
    cin >> l;
    for(int j=0;j<l;j++) {
      ll k;
      cin >> k;
      a[i].push_back(k);
    }
  }
  map<ll, ll> mp;
  mp[x] = 1;
  for(int i=0;i<n;i++) {
    map<ll, ll> tmp;
    for(int j=0;j<a[i].size();j++) {
      for(auto it=mp.rbegin();it!=mp.rend();it++) {
        if(it->first % a[i][j]) continue;
        tmp[it->first / a[i][j]] += it->second;
      }
    }
    mp.swap(tmp);
  }
  cout << mp[1] << endl;
  return 0;
}
